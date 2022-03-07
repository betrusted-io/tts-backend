#![cfg_attr(target_os = "none", no_std)]

pub mod api;
pub use api::*;
use xous::CID;
use xous_ipc::Buffer;
use num_traits::ToPrimitive;

#[derive(Debug)]
pub struct TtsBackend {
    conn: CID,
}
impl TtsBackend {
    pub fn new(xns: &xous_names::XousNames) -> Result<Self, xous::Error> {
        REFCOUNT.fetch_add(1, Ordering::Relaxed);
        let conn = xns.request_connection_blocking(api::SERVER_NAME_TTS_EXEC).expect("Can't connect to TtsBackend server");
        Ok(TtsBackend {
            conn,
        })
    }

    pub fn tts_config(&self, dedicated_sid: [u32; 4], opcode: u32, samples_per_cb: Option<u32>) -> Result<(), xous::Error> {
        let msg = TtsBackendConfig {
            sid: dedicated_sid,
            op: opcode,
            samples_per_cb,
        };
        let buf = Buffer::into_buf(msg).or(Err(xous::Error::InternalError))?;
        buf.lend(self.conn, TtsBeOpcode::RegisterCb.to_u32().unwrap()).map(|_| ())
    }

    pub fn tts_simple(&self, text: &str) -> Result<(), xous::Error> {
        let msg = TtsBackendMsg {
            text: xous_ipc::String::from_str(text),
        };
        let buf = Buffer::into_buf(msg).or(Err(xous::Error::InternalError))?;
        buf.lend(self.conn, TtsBeOpcode::StrToWav.to_u32().unwrap()).map(|_| ())
    }
}

use core::sync::atomic::{AtomicU32, Ordering};
static REFCOUNT: AtomicU32 = AtomicU32::new(0);
impl Drop for TtsBackend {
    fn drop(&mut self) {
        // the connection to the server side must be reference counted, so that multiple instances of this object within
        // a single process do not end up de-allocating the CID on other threads before they go out of scope.
        // Note to future me: you want this. Don't get rid of it because you think, "nah, nobody will ever make more than one copy of this object".
        if REFCOUNT.fetch_sub(1, Ordering::Relaxed) == 1 {
            unsafe{xous::disconnect(self.conn).unwrap();}
        }
        // if there was object-specific state (such as a one-time use server for async callbacks, specific to the object instance),
        // de-allocate those items here. They don't need a reference count because they are object-specific
    }
}