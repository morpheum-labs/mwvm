//! Store-context host function — `morpheum_store_context`.
//!
//! Persists arbitrary binary blobs into the agent's persistent KV store.

use super::prelude::*;

/// Register `morpheum_store_context` with the wasmtime [`Linker`].
///
/// # Errors
///
/// Returns [`MwvmError::HostRegistration`] if wasmtime rejects the binding.
#[allow(clippy::cast_sign_loss)]
pub fn register(linker: &mut Linker<StoreContext>, registry: &HostRegistry) -> Result<()> {
    let reg = registry.clone();
    linker
        .func_wrap(
            HOST_NAMESPACE,
            HOST_STORE_CONTEXT,
            move |mut caller: Caller<'_, StoreContext>,
                  key_ptr: i32,
                  key_len: i32,
                  blob_ptr: i32,
                  blob_len: i32|
                  -> anyhow::Result<i32> {
                if key_len <= 0 || key_len > 2048 {
                    anyhow::bail!("key length must be 1..2048");
                }
                if blob_len <= 0 || blob_len > 10 * 1024 * 1024 {
                    anyhow::bail!("blob length must be 1..10 MiB");
                }

                let mem = caller
                    .get_export("memory")
                    .and_then(wasmtime::Extern::into_memory)
                    .ok_or_else(|| anyhow::anyhow!("memory export not found"))?;

                let (key, blob) = {
                    let data = mem.data(&caller);

                    let ks = key_ptr as usize;
                    let ke = ks + key_len as usize;
                    let key = data
                        .get(ks..ke)
                        .ok_or_else(|| anyhow::anyhow!("invalid key ptr/len"))?
                        .to_vec();

                    let bs = blob_ptr as usize;
                    let be = bs + blob_len as usize;
                    let blob = data
                        .get(bs..be)
                        .ok_or_else(|| anyhow::anyhow!("invalid blob ptr/len"))?
                        .to_vec();

                    (key, blob)
                };

                reg.memory
                    .store(&key, blob)
                    .map_err(|e| anyhow::anyhow!("store failed: {e}"))?;

                Ok(0) // success
            },
        )
        .map_err(|e| MwvmError::HostRegistration {
            name: HOST_STORE_CONTEXT,
            source: e,
        })?;

    Ok(())
}
