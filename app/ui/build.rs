use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
	let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

	let mut current = manifest_dir.as_path();
	let mut env_path = None;

	loop {
		let candidate = current.join(".env");
		if candidate.exists() {
			env_path = Some(candidate);
			break;
		}

		match current.parent() {
			Some(parent) => current = parent,
			None => break,
		}
	}

	if let Some(path) = env_path {
		if let Err(e) = dotenv::from_path(&path) {
			eprintln!("Error: Failed to load .env file: {e}");
			process::exit(1);
		}
	} else {
		eprintln!("Error: .env file not found in workspace root or parent directories");
		process::exit(1);
	}

	for (key, value) in env::vars() {
		let rust_var_name = if cfg!(debug_assertions) {
			key.strip_prefix("DEV_").map_or_else(|| if key.starts_with("PROD_") { None } else { Some(key.clone()) }, |stripped| Some(stripped.to_string()))
		} else if let Some(stripped) = key.strip_prefix("PROD_") {
			Some(stripped.to_string())
		} else if !key.starts_with("DEV_") {
			Some(key.clone())
		} else {
			None
		};

		if let Some(var_name) = rust_var_name {
			println!("cargo:rustc-env={var_name}={value}");
		}
	}

	println!("cargo:rerun-if-changed=.env");
}
