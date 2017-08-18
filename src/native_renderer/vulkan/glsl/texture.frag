// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/native_renderer/vulkan/glsl/texture.frag

#version 450
#extension GL_ARB_separate_shader_objects : enable // TODO: is needed?

layout (binding = 1) uniform sampler2D tex;

layout (location = 0) in vec4 texcoord;

layout (location = 0) out vec4 uFragColor;

void main() {
	vec4 sampled = texture(tex, texcoord.xy);
	uFragColor = vec4(sampled.rgb, sampled.a * texcoord.z);
}
