// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native_renderer/vulkan/glsl/solid.frag

#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform UniformBuffer {
	mat4 models_tfm; // The Models' Transform Matrix
	vec4 color;
	int has_camera;
} uniforms;
layout (binding = 1) uniform Camera {
	mat4 matrix; // The Camera's Transform & Projection Matrix
} camera;
layout (binding = 2) uniform Fog {
	vec4 fog; // The fog color.
	vec2 range; // The range of fog (fog to far clip)
} fog;

layout (location = 0) in vec4 in_color;
layout (location = 1) in float z;

layout (location = 0) out vec4 frag_color;

void main() {
	if(in_color.a <= 0.0) {
		discard;
	} else if(uniforms.has_camera == 2) {
		// Fog Calculation
		float linear = clamp((z-fog.range.x) / fog.range.y, 0.0, 1.0);
		float curved = linear * linear * linear;
		frag_color = mix(in_color, fog.fog, curved);
	} else {
		frag_color = in_color;
	}
}
