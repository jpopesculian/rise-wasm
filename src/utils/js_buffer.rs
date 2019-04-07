use alloc::prelude::*;
use core::ops::Deref;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsBuffer {
    #[serde(rename = "type")]
    data_type: String,
    data: Vec<u8>,
}

impl Deref for JsBuffer {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl JsBuffer {
    pub fn new(data: Vec<u8>) -> JsBuffer {
        JsBuffer {
            data_type: String::from("Buffer"),
            data,
        }
    }
}

impl<T> From<T> for JsBuffer
where
    T: Into<Vec<u8>>,
{
    fn from(val: T) -> JsBuffer {
        JsBuffer::new(val.into())
    }
}
