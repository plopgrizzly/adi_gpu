/**
 * adi_screen - Aldaron's Device Interface - Screen - "build.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

extern crate gcc;

#[cfg(target_os = "linux")]
fn link() {
	println!("cargo:rustc-link-lib=vulkan");
}

#[cfg(target_os = "windows")]
fn link() {
	println!("cargo:rustc-link-args=-Wl,--subsystem,windows");
	println!("cargo:rustc-link-search=C:/Windows/SYSWOW64/");
	println!("cargo:rustc-link-search=C:/Windows/System32/");
	println!("cargo:rustc-link-lib=vulkan-1");
}

fn main() {
	gcc::Config::new().file("native/vw.c").flag("-Wall").flag("-Werror").compile("libaldaronvw.a");
	link();
}
