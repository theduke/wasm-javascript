fn main() -> Result<(), anyhow::Error> {
    // Create the JsHost.
    // The host is expensive to construct and so should be kept for the duration
    // of the program.
    eprintln!("Building JsHost...");
    let host = wasmtime_js::JsHost::new()?;

    // Create a new isolated Javascript context.
    // The supplied function handles guest <-> host interop.
    // The javascript can call the `hostcallstr()` function with a command and
    // a payload, both strings.
    // The handler returns a string.
    //
    // This allows (somewhat) flexible interop by choosing a custom serialization
    // format like JSON.
    eprintln!("Creating context...");
    let mut ctx = host.build_context(|command, _payload| {
        // Return value is a json serialized Result<String, String>.
        // Javascript can JSON.parse() and interpret the result.
        let res: Result<String, String> = match command {
            "hostname" => Ok("my-hostname.network".to_string()),
            other => Err(format!("error: unknown command {}", other)),
        };

        serde_json::to_string(&res).unwrap()
    })?;

    eprintln!("Evaluating...");

    // dbg!(ctx.eval("1 + 1").unwrap());

    // Add a Javascript helper function.
    //
    // NOTE: for some reason just declaring a regular function with 
    // `function hostname() { ... }` screws up the interpreter state.
    // So we add it to globalThis instead.
    ctx.eval(
        r#"
            globalThis.hostname = () => {
                const rawResult = hostcallstr("hostname", "");

                const json = JSON.parse(rawResult);

                if ('Err' in json) {
                    throw new Error(json.Err);
                } else if (!('Ok' in json)) {
                    throw new Error("Malformed response: missing 'Ok' key in " + json);
                }
                return json.Ok;
            };
    "#,
    )?;

    // // Invoke some Javascript code.
    // // Here we test that the `hostname` returned by the hostcall works.
    let hostname = ctx.eval("hostname()")?;

    eprintln!("Roundtripped hostname: {}", hostname);

    Ok(())
}
