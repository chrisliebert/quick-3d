// Copyright (C) 2016 Chris Liebert

extern crate cmake;

use std::fs;
use std::path::Path;
use std::process::Command;

#[cfg(target_os = "windows")]
fn copy_quick3d_shared_library_files() -> std::io::Result<()> {
	try!(fs::copy(Path::new("../../target/debug/quick3d.dll"), Path::new("quick3d.dll")));
	try!(fs::copy(Path::new("../../target/debug/quick3d.dll"), Path::new("wrapper/quick3d.dll")));
	try!(fs::copy(Path::new("../../target/debug/quick3d.dll.exp"), Path::new("wrapper/quick3d.dll.exp")));
	try!(fs::copy(Path::new("../../target/debug/quick3d.dll.lib"), Path::new("wrapper/quick3d.dll.lib")));
	try!(fs::copy(Path::new("../../target/debug/quick3d.pdb"), Path::new("wrapper/quick3d.pdb")));
	Ok(())
}

#[cfg(not(target_os = "windows"))]
fn copy_quick3d_shared_library_files() -> std::io::Result<()> {
	try!(fs::copy(Path::new("../../target/debug/libquick3d.so"), Path::new("libquick3d.so")));
	try!(fs::copy(Path::new("../../target/debug/libquick3d.so"), Path::new("wrapper/libquick3d.so")));
	Ok(())
}

fn copy_swig_interface_file() -> std::io::Result<()> {
	let quick3d_i_src = Path::new("../../quick3d.i");
	let quick3d_i_dst = Path::new("wrapper/quick3d.i");
	try!(fs::copy(quick3d_i_src, quick3d_i_dst));
	Ok(())
}

fn main() {
	// build Quick3D
	let build_cmd_output = Command::new("cargo")
		.arg("build")
		.current_dir("../../")
		.output()
		.unwrap();
	
	// Ensure cargo succeeded
	assert!(build_cmd_output.status.success());
	
	copy_quick3d_shared_library_files().unwrap();
	copy_swig_interface_file().unwrap();

	// cmake build for language wrappers
	let dst = cmake::Config::new("wrapper").build();
	println!("cargo:rustc-link-search=wrapper");
	println!("cargo:rustc-link-search=native={}", dst.display());
}
