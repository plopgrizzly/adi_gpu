/*
 * adi_screen - Aldaron's Device Interface - Screen - "glsl/texture.frag"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
*/

#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 1) uniform sampler2D tex;

layout (location = 0) in vec4 texcoord;

layout (location = 0) out vec4 uFragColor;

void main() {
	vec4 sampled = texture(tex, texcoord.xy);
	uFragColor = vec4(sampled.rgb, sampled.a * texcoord.z);
}
