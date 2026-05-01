//! Qylith Node Build Script
//!
//! This file configures the build process for the Qylith node,
//! including WASM runtime compilation.

use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_changed};

fn main() {
    generate_cargo_keys();

    rerun_if_git_changed("Cargo.toml");
    rerun_if_git_changed("runtime/Cargo.toml");
    rerun_if_git_changed("primitives/crypto/Cargo.toml");
    rerun_if_git_changed("runtime/src/pallets/aem/Cargo.toml");

    #[cfg(feature = "runtime-wasm")]
    {
        // Build WASM runtime
        wasm_builder::build_wasm(
            wasm_builder::SupportColumns::WasmBuilder,
            Some("runtime/wbuild".into()),
        )
        .expect("WASM build failed");
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/chain_spec.rs");
}
