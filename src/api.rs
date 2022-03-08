pub const SERVER_NAME_TTS_EXEC: &str     = "_Text To Speech Executable_ / (external C program)";
pub const MAX_WAV_BUF_SAMPLES: usize = 1024;

/// Back end opcode table
#[derive(num_derive::FromPrimitive, num_derive::ToPrimitive, Debug)]
pub enum TtsBeOpcode {
    /// Take a string and translate it to a WAV file
    StrToWav,
    /// Register callback routing for WAV data
    RegisterCb,
    /// Exit server
    Quit,
}

/// Backend-originated control messages that can be sent to the front end
#[derive(Debug, Copy, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum TtsBeControl {
    Abort,
    End,
}

/// Messages to the backend. Currently, just the text string can be sent, and it is
/// limited to 2048 characters in length.
#[derive(Debug, Copy, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct TtsBackendMsg {
    pub text: xous_ipc::String::<2048>,
}

/// Configuration data for the backend
#[derive(Debug, Copy, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct TtsBackendConfig {
    pub sid: [u32; 4],
    pub op: u32,
    pub samples_per_cb: Option<u32>,
}

/// Data returned by the backend to the `dedicated_sid`, routed to the opcode
/// as specified in the callback specifier.
#[derive(Debug, Copy, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct TtsBackendData {
    /// the 16-bit wave sample data being passed back.
    /// Can be no larger than MAX_WAV_BUF_SAMPLES, but is allowed to be smaller or even 0.
    pub data: [u16; MAX_WAV_BUF_SAMPLES],
    /// the actual length in the buffer
    pub len: u32,
    pub control: Option<TtsBeControl>,
}
