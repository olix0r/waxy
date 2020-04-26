use wasmer_runtime::{self as wasm, WasmPtr};

pub struct AdmitHeader {
    header: String,
    instance: wasm::Instance,
    value_ptr: WasmPtr<u8, wasm::Array>,
}

impl AdmitHeader {
    pub fn new(wasm: &[u8], header: String) -> wasm::error::Result<Self> {
        let import_object = wasm::imports! {};
        let instance = wasm::instantiate(&wasm, &import_object)?;
        let value_ptr = instance
            .func::<(), WasmPtr<u8, wasm::Array>>("admit_ptr")
            .expect("WASM does not implement `admit_ptr`")
            .call()
            .expect("`admit_ptr` panicked");
        Ok(Self {
            header,
            instance,
            value_ptr,
        })
    }
}

impl super::Admit for AdmitHeader {
    fn admit<B>(&mut self, req: &http::Request<B>) -> bool {
        let bytes = match req.headers().get(&self.header) {
            Some(value) => value.as_bytes(),
            None => &[],
        };
        let sz = bytes.len();
        write_bytes_and_null(bytes, self.value_ptr, self.instance.context_mut().memory(0));

        if let Ok(func) = self
            .instance
            .func::<(WasmPtr<u8, wasm::Array>, u16), u8>("admit")
        {
            if let Ok(1) = func.call(self.value_ptr, sz as u16) {
                return true;
            }
        }

        false
    }
}

fn write_bytes_and_null(bytes: &[u8], target: WasmPtr<u8, wasm::Array>, memory: &wasm::Memory) {
    let size = bytes.len() + 1;
    let mem = target.deref(&memory, 0, size as u32).expect("oom");
    for (byte, cell) in bytes.iter().map(|b| *b).chain(std::iter::once(0)).zip(mem) {
        cell.set(byte);
    }
}
