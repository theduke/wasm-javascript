witx_bindgen_wasmtime::export!("wasmtime_js/witx/js_api.witx");
witx_bindgen_wasmtime::import!("wasmtime_js/witx/host_api.witx");

static WASM_SPIDERMONKEY: &'static [u8] = include_bytes!("../wasm/spidermonkey.wasm");
static WASM_GLUE: &'static [u8] = include_bytes!("../wasm/js.wasm");

#[derive(Clone)]
pub struct JsHost {
    engine: wasmtime::Engine,
    spidermonkey_module: wasmtime::Module,
    glue_module: wasmtime::Module,
}

impl JsHost {
    pub fn new() -> Result<Self, anyhow::Error> {
        let mut config = wasmtime::Config::default();
        config
            .wasm_module_linking(true)
            .wasm_multi_memory(true)
            .wasm_reference_types(true)
            .cache_config_load_default()?;
        let engine = wasmtime::Engine::new(&config)?;

        // Load WASM modules.
        let spidermonkey_module = wasmtime::Module::new(&engine, WASM_SPIDERMONKEY)?;
        let glue_module = wasmtime::Module::new(&engine, WASM_GLUE)?;

        Ok(Self {
            engine,
            glue_module,
            spidermonkey_module,
        })
    }

    pub fn build_context<F>(&self, handler: F) -> Result<JsContext, anyhow::Error>
    where
        F: Fn(&str, &str) -> String + 'static,
    {
        let mut linker = wasmtime::Linker::<State>::new(&self.engine);

        let wasi = wasmtime_wasi::sync::WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();
        let state = State {
            wasi,
            host_api: HostApi {
                callback: Box::new(handler),
            },
            js_api: js_api::JsApiData {},
        };
        let mut store = wasmtime::Store::new(&self.engine, state);

        // Add wasi
        wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi)?;
        // Add spidermonkey.
        linker.module(&mut store, "spidermonkey", &self.spidermonkey_module)?;

        // Add glue code.
        host_api::add_host_api_to_linker(&mut linker, |s| &mut s.host_api)?;

        let (js, instance) =
            js_api::JsApi::instantiate(&mut store, &self.glue_module, &mut linker, |s| {
                &mut s.js_api
            })?;

        // Call initialization code.
        instance
            .get_typed_func::<(), (), _>(&mut store, "wizer.initialize")?
            .call(&mut store, ())?;

        Ok(JsContext {
            store,
            _instance: instance,
            js,
        })
    }
}

pub struct JsContext {
    store: wasmtime::Store<State>,
    _instance: wasmtime::Instance,
    js: js_api::JsApi<State>,
}

impl JsContext {
    pub fn eval(&mut self, js_code: &str) -> Result<String, wasmtime::Trap> {
        self.js.js_eval(&mut self.store, js_code)
    }
}

pub type HostcallHandler = Box<dyn Fn(&str, &str) -> String>;

struct HostApi {
    callback: HostcallHandler,
}

impl host_api::HostApi for HostApi {
    // FIXME: somehow the generated code erronously passes the javascript code as
    // the first argument!  Need to investigate witx-bindgen-gen-spidermonkey code.
    fn hostcall_str(&mut self, _code: &str, command: &str, data: &str) -> String {
        (&*self.callback)(command, data)
    }
}

struct State {
    wasi: wasmtime_wasi::WasiCtx,
    host_api: HostApi,
    js_api: js_api::JsApiData,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        eprintln!("building host...");
        let host = JsHost::new().unwrap();
        eprintln!("Host built!");
        eprintln!("buuilding context...");
        let mut ctx = host
            .build_context(|_cmd, _data| "hello".to_string())
            .unwrap();
        eprintln!("context built!");

        assert_eq!(ctx.eval("1+2").unwrap(), "3");
        assert_eq!(ctx.eval(r#"hostcall_str("a", "b")"#).unwrap(), "hello");
    }
}
