// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native_renderer/vulkan/glsl/tinted.vert

#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform UniformBuffer {
	mat4 models_tfm; // The Models' Transform Matrix
	vec4 color;
} uniforms;

layout (binding = 1) uniform Camera {
	mat4 matrix; // The Camera's Transform & Projection Matrix
} camera;

layout (location = 0) in vec4 pos;
layout (location = 1) in vec4 texpos;

layout (location = 0) out vec4 texcoord;
layout (location = 1) out vec4 tint;

void main() {
	texcoord = texpos;
	tint = uniforms.color;
	gl_Position = camera.matrix * (uniforms.models_tfm * pos);
}
