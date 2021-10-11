//! Binary that generates a js.wasm file based on .witx interface type definitions.
//!
//! Uses witx-bindgen

use witx_bindgen_gen_core::Generator;

fn main() {
    gen().unwrap();
}

fn gen() -> Result<(), anyhow::Error> {
    let workspace_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf();
    let witx_dir = workspace_dir.join("wasmtime_js").join("witx");
    let output_dir = workspace_dir.join("wasmtime_js").join("wasm");

    let mut gen = witx_bindgen_gen_spidermonkey::SpiderMonkeyWasm::new(
        "js.js",
        "
import * as host_api from 'host_api';

globalThis.hostcall_str = (command, payload) => {
    return host_api.hostcall_str(command, payload);
};

export function js_eval(code) {
    // Wrap execution in a closure.
    // Seems to prevent some issues.
    return (() => {
        return globalThis.eval(code).toString();
    })();
}

",
    );
    let mut files = Default::default();

    let js_api = witx2::Interface::parse_file(witx_dir.join("js_api.witx")).unwrap();
    let host_api = witx2::Interface::parse_file(witx_dir.join("host_api.witx")).unwrap();

    gen.import_spidermonkey(true);
    gen.generate_all(&[host_api], &[js_api], &mut files);

    if !output_dir.is_dir() {
        // std::fs::remove_dir_all(&out).unwrap();
        std::fs::create_dir_all(&output_dir).unwrap();
    }

    for (name, wasm_code) in files.iter() {
        let path = output_dir.join(name);
        std::fs::write(&path, wasm_code).unwrap();

        // Initialize with wizer to reduce startup time.
        // (doesn't work because of no reference type support...)

        // let name_uninit = format!("{}.uninitialized", name);
        // let path_uninit = out.join(name_uninit);
        // let path_init = out.join(name);
        // let code = std::process::Command::new("wizer")
        //     .args(&[
        //           "--wasm-module-linking",
        //           "true",
        //           "--wasm-multi-memory",
        //           "true",

        //     ])
        //     .arg(&path_uninit)
        //     .arg("-o")
        //     .arg(&path_init)
        //     .spawn()?
        //     .wait()?;
        // if !code.success() {
        //     anyhow::bail!("Wizer failed");
        // }
        // std::fs::remove_file(path_uninit)?;
        // eprintln!("Generated {}", path_init.display());
    }

    Ok(())
}
