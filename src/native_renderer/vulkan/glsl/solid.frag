// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/native_renderer/vulkan/glsl/solid.frag

#version 450

layout (location = 0) in vec4 inColor;
layout (location = 0) out vec4 outColor;

void main() {
	outColor = inColor;
}
