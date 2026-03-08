//! Vector search host function — `morpheum_vector_search`.
//!
//! Approximate nearest-neighbour search over the agent's persistent vector
//! memory. Output is a packed array of `(f32 score, u64 id)` tuples.

use super::prelude::*;
use std::mem;

/// Packed result written back to WASM linear memory.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct PackedSearchResult {
    score: f32,
    _pad: u32,
    entry_id: u64,
}

/// Register `morpheum_vector_search` with the wasmtime [`Linker`].
///
/// # Errors
///
/// Returns [`MwvmError::HostRegistration`] if wasmtime rejects the binding.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn register(linker: &mut Linker<StoreContext>, registry: &HostRegistry) -> Result<()> {
    let reg = registry.clone();
    linker
        .func_wrap(
            HOST_NAMESPACE,
            HOST_VECTOR_SEARCH,
            move |mut caller: Caller<'_, StoreContext>,
                  query_ptr: i32,
                  query_len: i32,
                  k: i32,
                  out_ptr: i32,
                  out_max_len: i32|
                  -> anyhow::Result<i32> {
                if k <= 0 || k > 512 {
                    anyhow::bail!("k must be in 1..=512");
                }
                if query_len <= 0 {
                    anyhow::bail!("empty query vector");
                }

                let mem = caller
                    .get_export("memory")
                    .and_then(wasmtime::Extern::into_memory)
                    .ok_or_else(|| anyhow::anyhow!("memory export not found"))?;

                // Read query vector (scoped borrow).
                let query: Vec<f32> = {
                    let data = mem.data(&caller);
                    let byte_len = (query_len as usize) * mem::size_of::<f32>();
                    let start = query_ptr as usize;
                    let slice = data
                        .get(start..start + byte_len)
                        .ok_or_else(|| anyhow::anyhow!("invalid query ptr/len"))?;
                    bytemuck::cast_slice::<u8, f32>(slice).to_vec()
                };

                // Delegate to LocalMemory.
                let results = reg.memory.search(&query, k as usize);

                // Pack results.
                let item_size = mem::size_of::<PackedSearchResult>();
                let max_items = (out_max_len as usize) / item_size;
                let packed: Vec<PackedSearchResult> = results
                    .into_iter()
                    .take(max_items)
                    .map(|r| PackedSearchResult {
                        score: r.score,
                        _pad: 0,
                        entry_id: r.id,
                    })
                    .collect();
                let out_bytes = bytemuck::cast_slice::<PackedSearchResult, u8>(&packed);

                // Write back (scoped mutable borrow).
                {
                    let data = mem.data_mut(&mut caller);
                    let start = out_ptr as usize;
                    let dest = data
                        .get_mut(start..start + out_max_len as usize)
                        .ok_or_else(|| anyhow::anyhow!("output buffer too small"))?;
                    let copy_len = out_bytes.len().min(dest.len());
                    dest[..copy_len].copy_from_slice(&out_bytes[..copy_len]);
                    Ok(copy_len as i32)
                }
            },
        )
        .map_err(|e| MwvmError::HostRegistration {
            name: HOST_VECTOR_SEARCH,
            source: e,
        })?;

    Ok(())
}
