use core::str;

pub type Buffer = Vec<u8>;

pub fn buf_to_string(buf: &Buffer) -> Result<String, RustChatError> {
    let trimed_content: Vec<u8> = buf.iter().filter(|c| **c != 0).map(|&x| x).collect();


    let content = str::from_utf8(&trimed_content).map_err(|_| RustChatError::BufferConversionError( 
        format!("Could not convert buffer into string"))
    )?;

    return Ok(content.trim().to_string());
}



#[derive(Debug)]
pub enum RustChatError {
    BufferConversionError(String)
}

impl std::error::Error for RustChatError {}

impl std::fmt::Display for RustChatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BufferConversionError(msg) => write!(f, "Buffer conversion error: {}", msg),
        }
    }
}

