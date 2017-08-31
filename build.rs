// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// build.rs

extern crate gcc;

#[cfg(target_os = "linux")]
fn link() {
	// TODO: Remove
	println!("cargo:rustc-link-lib=vulkan");
}

#[cfg(target_os = "windows")]
fn link() {
	println!("cargo:rustc-link-args=-Wl,--subsystem,windows");
	println!("cargo:rustc-link-search=C:/Windows/SYSWOW64/");
	println!("cargo:rustc-link-search=C:/Windows/System32/");
	// TODO: Remove
	println!("cargo:rustc-link-lib=vulkan-1");
}

fn main() {
	gcc::Config::new().file("native/vw.c").flag("-Wall").flag("-Werror").compile("libaldaronvw.a");
	link();
}
