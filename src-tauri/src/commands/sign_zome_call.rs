use holochain::core::{
    CapSecret, CellId, ExternIO, FunctionName, Timestamp, ZomeCallUnsigned, ZomeName,
};
use holochain_client::{AgentPubKey, ZomeCall};
use serde::Deserialize;

use crate::app_state::AppState;

#[tauri::command]
pub async fn sign_zome_call(
    state: tauri::State<'_, AppState>,
    zome_call_unsigned: ZomeCallUnsignedTauri,
) -> Result<ZomeCall, String> {
    let zome_call_unsigned_converted: ZomeCallUnsigned = zome_call_unsigned.into();

    let keystore = state.meta_lair_client.lock().await;

    let signed_zome_call =
        ZomeCall::try_from_unsigned_zome_call(&keystore, zome_call_unsigned_converted)
            .await
            .map_err(|e| format!("Failed to sign zome call: {}", e))?;

    Ok(signed_zome_call)
}

/// The version of an unsigned zome call that's compatible with the serialization
/// behavior of tauri's IPC channel (serde serialization)
/// nonce is a byte array [u8, 32] because holochain's nonce type seems to
/// have "non-serde" deserialization behavior.
#[derive(Deserialize, Debug, Clone)]
pub struct ZomeCallUnsignedTauri {
    pub provenance: AgentPubKey,
    pub cell_id: CellId,
    pub zome_name: ZomeName,
    pub fn_name: FunctionName,
    pub cap_secret: Option<CapSecret>,
    pub payload: ExternIO,
    pub nonce: [u8; 32],
    pub expires_at: Timestamp,
}

impl Into<ZomeCallUnsigned> for ZomeCallUnsignedTauri {
    fn into(self) -> ZomeCallUnsigned {
        ZomeCallUnsigned {
            provenance: self.provenance,
            cell_id: self.cell_id,
            zome_name: self.zome_name,
            fn_name: self.fn_name,
            cap_secret: self.cap_secret,
            payload: self.payload,
            nonce: self.nonce.into(),
            expires_at: self.expires_at,
        }
    }
}
