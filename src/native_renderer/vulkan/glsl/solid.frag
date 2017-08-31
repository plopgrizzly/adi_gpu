// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native_renderer/vulkan/glsl/solid.frag

#version 450

layout (location = 0) in vec4 inColor;
layout (location = 0) out vec4 outColor;

void main() {
	outColor = inColor;
}
