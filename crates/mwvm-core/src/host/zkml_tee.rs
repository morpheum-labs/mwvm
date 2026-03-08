//! zkML + TEE verification host functions.
//!
//! Rich off-chain simulation: structural checks, timestamp freshness (TEE),
//! public-inputs hash consistency (zkML), and deterministic pass/fail results.

use super::prelude::*;

/// Register `morpheum_zkml_verify` with the wasmtime [`Linker`].
///
/// # Errors
///
/// Returns [`MwvmError::HostRegistration`] if wasmtime rejects the binding.
#[allow(clippy::cast_sign_loss)]
pub fn register_zkml(linker: &mut Linker<StoreContext>, _registry: &HostRegistry) -> Result<()> {
    linker
        .func_wrap(
            HOST_NAMESPACE,
            HOST_ZKML_VERIFY,
            move |mut caller: Caller<'_, StoreContext>,
                  proof_ptr: i32,
                  proof_len: i32|
                  -> anyhow::Result<i32> {
                #[cfg(not(feature = "tee-simulation"))]
                {
                    let _ = (&mut caller, proof_ptr, proof_len);
                    anyhow::bail!("zkML verification requires the tee-simulation feature");
                }

                #[cfg(feature = "tee-simulation")]
                {
                    let mem = caller
                        .get_export("memory")
                        .and_then(wasmtime::Extern::into_memory)
                        .ok_or_else(|| anyhow::anyhow!("memory export not found"))?;

                    let proof: ZkmlProof = {
                        let data = mem.data(&caller);
                        let start = proof_ptr as usize;
                        let end = start + proof_len as usize;
                        let slice = data
                            .get(start..end)
                            .ok_or_else(|| anyhow::anyhow!("invalid proof ptr/len"))?;
                        *bytemuck::from_bytes(slice)
                    };

                    proof
                        .validate()
                        .map_err(|e| anyhow::anyhow!("invalid proof: {e}"))?;

                    Ok(1) // 1 = valid
                }
            },
        )
        .map_err(|e| MwvmError::HostRegistration {
            name: HOST_ZKML_VERIFY,
            source: e,
        })?;

    Ok(())
}

/// Register `morpheum_tee_verify` with the wasmtime [`Linker`].
///
/// # Errors
///
/// Returns [`MwvmError::HostRegistration`] if wasmtime rejects the binding.
#[allow(clippy::cast_sign_loss)]
pub fn register_tee(linker: &mut Linker<StoreContext>, _registry: &HostRegistry) -> Result<()> {
    linker
        .func_wrap(
            HOST_NAMESPACE,
            HOST_TEE_VERIFY,
            move |mut caller: Caller<'_, StoreContext>,
                  att_ptr: i32,
                  att_len: i32|
                  -> anyhow::Result<i32> {
                #[cfg(not(feature = "tee-simulation"))]
                {
                    let _ = (&mut caller, att_ptr, att_len);
                    anyhow::bail!("TEE verification requires the tee-simulation feature");
                }

                #[cfg(feature = "tee-simulation")]
                {
                    let mem = caller
                        .get_export("memory")
                        .and_then(wasmtime::Extern::into_memory)
                        .ok_or_else(|| anyhow::anyhow!("memory export not found"))?;

                    let att: TeeAttestation = {
                        let data = mem.data(&caller);
                        let start = att_ptr as usize;
                        let end = start + att_len as usize;
                        let slice = data
                            .get(start..end)
                            .ok_or_else(|| anyhow::anyhow!("invalid attestation ptr/len"))?;
                        *bytemuck::from_bytes(slice)
                    };

                    att.validate()
                        .map_err(|e| anyhow::anyhow!("invalid attestation: {e}"))?;

                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();

                    // Attestation must be recent (within 24 h).
                    if att.timestamp > now || now.saturating_sub(att.timestamp) > 86_400 {
                        return Ok(0); // 0 = stale
                    }

                    Ok(1) // 1 = valid
                }
            },
        )
        .map_err(|e| MwvmError::HostRegistration {
            name: HOST_TEE_VERIFY,
            source: e,
        })?;

    Ok(())
}
