use alloc::prelude::*;
use core::ops::Deref;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsBuffer {
    data: Vec<u8>,
}

impl Deref for JsBuffer {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
