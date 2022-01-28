use js_sys::{ArrayBuffer, Object, Reflect, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type WorkersKvJs;

    #[wasm_bindgen(structural, method, catch)]
    pub async fn put(
        this: &WorkersKvJs,
        k: JsValue,
        v: JsValue,
        // options: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(structural, method, catch)]
    pub async fn get(
        this: &WorkersKvJs,
        key: JsValue,
        options: JsValue,
    ) -> Result<JsValue, JsValue>;
}

pub(crate) struct WorkersKv {
    pub kv: WorkersKvJs,
}

#[allow(dead_code)]
impl WorkersKv {
    pub async fn put_text(&self, key: &str, value: &str, ttl: u64) -> Result<(), JsValue> {
        let options = Object::new();
        Reflect::set(&options, &"expirationTtl".into(), &(ttl as f64).into())?;
        self.kv
            .put(JsValue::from_str(key), value.into()) //, options.into())
            .await?;
        Ok(())
    }

    pub async fn put_vec(&self, key: &str, value: &[u8], ttl: u64) -> Result<(), JsValue> {
        let options = Object::new();
        Reflect::set(&options, &"expirationTtl".into(), &(ttl as f64).into())?;
        let typed_array = Uint8Array::new_with_length(value.len() as u32);
        typed_array.copy_from(value);
        self.kv
            .put(
                JsValue::from_str(key),
                typed_array.buffer().into(),
                // options.into(),
            )
            .await?;
        Ok(())
    }

    pub async fn get_text(&self, key: &str) -> Result<Option<String>, JsValue> {
        let options = Object::new();
        Reflect::set(&options, &"type".into(), &"text".into())?;
        Ok(self
            .kv
            .get(JsValue::from_str(key), options.into())
            .await?
            .as_string())
    }

    pub async fn get_vec(&self, key: &str) -> Result<Option<Vec<u8>>, JsValue> {
        let options = Object::new();
        Reflect::set(&options, &"type".into(), &"arrayBuffer".into())?;
        let value = self.kv.get(JsValue::from_str(key), options.into()).await?;
        if value.is_null() {
            Ok(None)
        } else {
            let buffer = ArrayBuffer::from(value);
            let typed_array = Uint8Array::new_with_byte_offset(&buffer, 0);
            let mut v = vec![0; typed_array.length() as usize];
            typed_array.copy_to(v.as_mut_slice());
            Ok(Some(v))
        }
    }
}
