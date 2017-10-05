// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native_renderer/vulkan/glsl/solid.vert

#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform UniformBuffer {
	mat4 matrix_transform;
	vec4 color;
} uniforms;

layout (location = 0) in vec4 pos;
layout (location = 0) out vec4 inColor;

void main() {
	inColor = uniforms.color;
	gl_Position = uniforms.matrix_transform * pos;
}
