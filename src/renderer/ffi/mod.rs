// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/ffi/mod.rs

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "android"))]
#[link(name = "vulkan-1")]
pub mod vulkan;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub mod opengl;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod metal;

pub mod agi;
