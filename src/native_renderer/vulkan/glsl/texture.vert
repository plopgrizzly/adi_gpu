/*
 * adi_screen - Aldaron's Device Interface - Screen - "glsl/texture.vert"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
*/

#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform UniformBuffer {
	mat4 mtf;
} ub;

layout (location = 0) in vec4 pos;
layout (location = 1) in vec4 texpos;

layout (location = 0) out vec4 texcoord;

void main() {
	texcoord = texpos;
	gl_Position = ub.mtf * pos;
}
