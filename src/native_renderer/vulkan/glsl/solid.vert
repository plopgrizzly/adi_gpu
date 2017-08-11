// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/native_renderer/vulkan/glsl/solid.vert

#version 450

layout (binding = 0) uniform UniformBuffer {
	vec4 color;
} ub;

layout (location = 0) in vec4 pos;
layout (location = 0) out vec4 inColor;

void main() {
	inColor = ub.color;
	gl_Position = pos;
}
