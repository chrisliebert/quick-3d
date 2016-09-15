extern crate cmake;

fn main() {
	// cmake build for language wrappers
	let dst = cmake::Config::new("dependencies").build();
	println!("cargo:rustc-link-search=dependencies");
	println!("cargo:rustc-link-search=native={}", dst.display());
}
