// Aldaron's Device Interface / GPU
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/native_renderer/vulkan/glsl/tinted.frag

#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 2) uniform sampler2D tex;

layout (location = 0) in vec4 texcoord;
layout (location = 1) in vec4 tint;

layout (location = 0) out vec4 uFragColor;

void main() {
	vec4 sampled = texture(tex, texcoord.xy);
	uFragColor = vec4(sampled.rgb * tint.rgb, 1.0);
}
