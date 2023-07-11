
use holochain::{conductor::ConductorHandle, prelude::kitsune_p2p::dependencies::kitsune_p2p_types::dependencies::lair_keystore_api::prelude::BinDataSized};
use holochain_types::prelude::ZomeCallUnsigned;
use holochain_zome_types::{Signature, CellId, ZomeName, FunctionName, CapSecret, ExternIO, Timestamp};

use holochain_client::{AdminWebsocket, ZomeCall, AgentPubKey};


use serde::Deserialize;

use crate::errors::{AppResult, AppError};


#[tauri::command]
pub async fn sign_zome_call(
    conductor: tauri::State<'_, futures::lock::Mutex<ConductorHandle>>,
    zome_call_unsigned: ZomeCallUnsignedTauri,
) -> Result<ZomeCall, String> {
    let zome_call_unsigned_converted: ZomeCallUnsigned = zome_call_unsigned.into();

    let conductor = conductor.lock().await;
    let lair_client = conductor.keystore().lair_client();

    let pub_key = zome_call_unsigned_converted.provenance.clone();
    let mut pub_key_2 = [0; 32];
    pub_key_2.copy_from_slice(pub_key.get_raw_32());

    let data_to_sign = zome_call_unsigned_converted.data_to_sign().unwrap();
        // .map_err(|e| format!("Failed to get data to sign from unsigned zome call: {}", e))
        // .map_err(|e| AppError::SignZomeCallError(e))?;

    let sig = lair_client.sign_by_pub_key(
        BinDataSized::from(pub_key_2),
        None,
        data_to_sign,
    ).await.unwrap();
        // .map_err(|e| AppError::SignZomeCallError(e.to_string()))?;

    let signature = Signature(*sig.0);

    let signed_zome_call = ZomeCall {
        cell_id: zome_call_unsigned_converted.cell_id,
        zome_name: zome_call_unsigned_converted.zome_name,
        fn_name: zome_call_unsigned_converted.fn_name,
        payload: zome_call_unsigned_converted.payload,
        cap_secret: zome_call_unsigned_converted.cap_secret,
        provenance: zome_call_unsigned_converted.provenance,
        nonce: zome_call_unsigned_converted.nonce,
        expires_at: zome_call_unsigned_converted.expires_at,
        signature
    };

    Ok(signed_zome_call)
}


pub async fn get_admin_ws(admin_port: u16) -> AppResult<AdminWebsocket> {
	let admin_ws = AdminWebsocket::connect(format!(
		"ws://localhost:{}",
		admin_port
	))
	.await
	.map_err(|err| {
		AppError::AdminWebsocketError(format!("Could not connect to the admin interface: {}", err))
	})?;

	Ok(admin_ws)
}

pub fn vec_to_locked(mut pass_tmp: Vec<u8>) -> std::io::Result<sodoken::BufRead> {
  match sodoken::BufWrite::new_mem_locked(pass_tmp.len()) {
    	Err(e) => {
        	pass_tmp.fill(0);
        	Err(e.into())
      }
      Ok(p) => {
			{
				let mut lock = p.write_lock();
				lock.copy_from_slice(&pass_tmp);
				pass_tmp.fill(0);
			}
			Ok(p.to_read())
      }
  }
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


// Event-listener added to the window object to listten to CTRl + scroll events for altering the zoom factor of the webview
pub const ZOOM_ON_SCROLL: &str = r#"
	// Adding event listeners to adjust zoom level on Ctrl + scroll
	function increaseZoomLevel(amount) {
	const percentageString = document.body.style.zoom;
	let num = percentageString === "" ? 100 : parseInt(percentageString.slice(0, percentageString.length-1));
	let newVal = num + Math.round(amount) < 500 ? num + Math.round(amount) : 500;
	document.body.style.zoom = `${newVal}%`
	}
	function decreaseZoomLevel(amount) {
	const percentageString = document.body.style.zoom;
	let num = percentageString === "" ? 100 : parseInt(percentageString.slice(0, percentageString.length-1));
	let newVal = num - Math.round(amount) > 30 ? num - Math.round(amount) : 30;
	document.body.style.zoom = `${newVal}%`
	}
	window.onkeydown = (ev) => {
	if (ev.key === "Control") {
		window.onwheel = (ev) => {
		if (ev.deltaY > 0) {
			decreaseZoomLevel(10);
		} else if (ev.deltaY < 0) {
			increaseZoomLevel(10);
		}
		}
	}
	};
	window.onkeyup = (ev) => {
	if (ev.key === "Control") {
		window.onwheel = null;
	}
	}
"#;

