// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native_renderer/vulkan/glsl/solid.vert

#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform UniformBuffer {
	mat4 models_tfm; // The Models' Transform Matrix
	vec4 color;
} uniforms;
layout (binding = 1) uniform Camera {
	mat4 matrix; // The Camera's Transform & Projection Matrix
	vec4 fog; // The fog color.
	vec2 range; // The range of fog (fog to far clip)
} camera;
layout (binding = 2) uniform Fog {
	vec4 fog; // The fog color.
	vec2 range; // The range of fog (fog to far clip)
} fog;

layout (location = 0) in vec4 pos;

layout (location = 0) out vec4 inColor;
layout (location = 1) out float z;

void main() {
	inColor = uniforms.color;
	gl_Position = camera.matrix * (uniforms.models_tfm * vec4(pos.xyz,1.0));
	z = length(gl_Position.xyz);
}
