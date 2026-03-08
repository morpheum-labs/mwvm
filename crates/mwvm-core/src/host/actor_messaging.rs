//! Actor messaging host function — `morpheum_actor_send`.
//!
//! Enables WASM agents to send messages to other agents. Messages are persisted
//! in the shared [`LocalMemory`] under deterministic mailbox keys.

use super::prelude::*;

/// Register `morpheum_actor_send` with the wasmtime [`Linker`].
///
/// # Errors
///
/// Returns [`MwvmError::HostRegistration`] if wasmtime rejects the binding.
#[allow(clippy::cast_sign_loss)]
pub fn register(linker: &mut Linker<StoreContext>, registry: &HostRegistry) -> Result<()> {
    let reg = registry.clone();
    linker
        .func_wrap(
            "morpheum",
            "morpheum_actor_send",
            move |mut caller: Caller<'_, StoreContext>,
                  target_ptr: i32,
                  target_len: i32,
                  msg_ptr: i32,
                  msg_len: i32|
                  -> anyhow::Result<i32> {
                if target_len <= 0 || target_len > 256 {
                    anyhow::bail!("target agent ID length must be 1..256 bytes");
                }
                if msg_len <= 0 || msg_len > 1024 * 1024 {
                    anyhow::bail!("message payload must be 1 byte .. 1 MiB");
                }

                let mem = caller
                    .get_export("memory")
                    .and_then(wasmtime::Extern::into_memory)
                    .ok_or_else(|| anyhow::anyhow!("memory export not found"))?;

                let (target_bytes, msg_bytes) = {
                    let data = mem.data(&caller);

                    let ts = target_ptr as usize;
                    let te = ts + target_len as usize;
                    let target = data
                        .get(ts..te)
                        .ok_or_else(|| anyhow::anyhow!("invalid target ptr/len"))?
                        .to_vec();

                    let ms = msg_ptr as usize;
                    let me = ms + msg_len as usize;
                    let msg = data
                        .get(ms..me)
                        .ok_or_else(|| anyhow::anyhow!("invalid message ptr/len"))?
                        .to_vec();

                    (target, msg)
                };

                // Deterministic mailbox key.
                let target_hash: [u8; 32] = blake3::hash(&target_bytes).into();
                let mailbox_key =
                    format!("messages/to/{}", hex::encode(target_hash)).into_bytes();

                reg.memory
                    .store(&mailbox_key, msg_bytes)
                    .map_err(|e| anyhow::anyhow!("failed to store message: {e}"))?;

                debug!(
                    target_hash = %hex::encode(&target_hash[..8]),
                    msg_size = msg_len,
                    "actor message delivered"
                );

                Ok(0) // success
            },
        )
        .map_err(|e| MwvmError::HostRegistration {
            name: "morpheum_actor_send",
            source: e,
        })?;

    Ok(())
}
