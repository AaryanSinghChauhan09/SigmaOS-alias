//! SigmaOS-alias — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_alias_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-alias: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-alias: ok");
}
