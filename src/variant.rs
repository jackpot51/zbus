use byteorder::ByteOrder;
use std::{error, fmt, str};

pub struct Variant {
    signature: String,
    value: Vec<u8>,
}

#[derive(Debug)]
pub enum VariantError {
    IncorrectType,
    InvalidUtf8,
    InsufficientData,
    UnsupportedType,
}

impl error::Error for VariantError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for VariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariantError::IncorrectType => write!(f, "incorrect type"),
            VariantError::InvalidUtf8 => write!(f, "invalid UTF-8"),
            VariantError::InsufficientData => write!(f, "insufficient data"),
            VariantError::UnsupportedType => write!(f, "unsupported type"),
        }
    }
}

// FIXME: Perhaps it'd be great not to copy here but that'd mean dealing with
//        lifetimes so let's do it later. :)
impl Variant {
    pub fn from_data(data: &[u8], signature: &str) -> Result<Self, VariantError> {
        let value = match signature {
            "u" => copy_u32(data)?,
            "s" | "o" => copy_string(data)?,
            "g" => copy_signature(data)?,
            _ => return Err(VariantError::UnsupportedType),
        };

        Ok(Self {
            value,
            signature: String::from(signature),
        })
    }

    pub fn from_string(string: &str) -> Self {
        Self {
            value: encode_string(string),
            signature: String::from("s"),
        }
    }

    pub fn from_object_path(path: &str) -> Self {
        Self {
            value: encode_string(path),
            signature: String::from("o"),
        }
    }

    pub fn from_signature_string(signature_string: &str) -> Self {
        Self {
            value: encode_signature(signature_string),
            signature: String::from("g"),
        }
    }

    pub fn from_u32(value: u32) -> Self {
        Self {
            value: encode_u32(value),
            signature: String::from("u"),
        }
    }

    // FIXME: Return an '&str'
    pub fn get_signature(&self) -> String {
        self.signature.clone()
    }

    // FIXME: Return a slice
    pub fn get_bytes(&self) -> Vec<u8> {
        self.value.clone()
    }

    // FIXME: Return an '&str'
    pub fn get_string(&self) -> Result<String, VariantError> {
        let i = match self.signature.as_str() {
            "s" | "o" => 4,
            "g" => 1,
            _ => return Err(VariantError::IncorrectType),
        };

        str::from_utf8(&self.value[i..])
            .map(|s| String::from(s))
            .map_err(|_| VariantError::InvalidUtf8)
    }

    pub fn get_u32(&self) -> Result<u32, VariantError> {
        if self.signature != "u" || self.value.len() < 4 {
            return Err(VariantError::IncorrectType);
        }

        Ok(byteorder::NativeEndian::read_u32(&self.value))
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }
}

fn copy_u32(data: &[u8]) -> Result<Vec<u8>, VariantError> {
    if data.len() < 4 {
        return Err(VariantError::InsufficientData);
    }

    Ok(data[0..4].into())
}

fn copy_string(data: &[u8]) -> Result<Vec<u8>, VariantError> {
    if data.len() < 4 {
        return Err(VariantError::InsufficientData);
    }

    let last_index = byteorder::NativeEndian::read_u32(data) as usize + 5;
    if data.len() < last_index {
        return Err(VariantError::InsufficientData);
    }

    Ok(data[0..last_index].into())
}

fn copy_signature(data: &[u8]) -> Result<Vec<u8>, VariantError> {
    if data.len() < 1 {
        return Err(VariantError::InsufficientData);
    }

    let last_index = data[0] as usize + 1;
    if data.len() < last_index {
        return Err(VariantError::InsufficientData);
    }

    Ok(data[0..last_index].into())
}

fn encode_u32(value: u32) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4);

    bytes.extend(&value.to_ne_bytes());

    bytes
}

fn encode_string(value: &str) -> Vec<u8> {
    let len = value.len();
    let mut bytes = Vec::with_capacity(5 + len);

    bytes.extend(&(len as u32).to_ne_bytes());
    bytes.extend(value.as_bytes());
    bytes.push(b'\0');

    bytes
}

fn encode_signature(value: &str) -> Vec<u8> {
    let len = value.len();
    let mut bytes = Vec::with_capacity(2 + len);

    bytes.push(len as u8);
    bytes.extend(value.as_bytes());
    bytes.push(b'\0');

    bytes
}