/**
 * Aldaron's Device Interface - "vw.h"
 * Copyright 2017 (c) Jeron Lau - Licensed under the GNU GENERAL PUBLIC LICENSE
**/

#include <string.h>
#include <stdio.h>
#include <stdlib.h>
// #include <unistd.h>

//#define VK_NO_PROTOTYPES
#include "vulkan/vulkan.h"

typedef struct {
	VkInstance instance; // Vulkan instance
	VkSurfaceKHR surface; // Surface that we render to.
	uint32_t present_queue_index;
	VkQueue present_queue;
	VkPhysicalDevice gpu;
	VkDevice device; // The logical device
	VkCommandBuffer command_buffer;
	VkSwapchainKHR swapchain;
	uint32_t width, height; // Swapchain Dimensions.
	VkImage present_images[2]; // 2 for double-buffering
	VkFramebuffer frame_buffers[2]; // 2 for double-buffering
	VkFormat color_format;
	uint32_t image_count; // 1 (single-buffering) or 2 (double-buffering)
	VkFence submit_fence; // The submit fence
	VkImageView present_image_views[2]; // 2 for double-buffering
	VkImage depth_image;
	VkImageView depth_image_view;
	VkDeviceMemory depth_image_memory;
	VkRenderPass render_pass;
	uint32_t next_image_index;
	VkSemaphore presenting_complete_sem, rendering_complete_sem;
	VkDeviceSize offset;
	VkPresentModeKHR present_mode;
	uint8_t sampled;
} vw_t;

typedef struct {
	VkShaderModule vertex;
	VkShaderModule fragment;
	uint32_t textures;
	uint8_t has_data;
} vw_shader_t;

typedef struct {
	VkPipeline pipeline;
	VkDescriptorSetLayout descsetlayout;
	VkPipelineLayout pipeline_layout;
} vw_pipeline_t;

/*typedef struct {
	VkBuffer matrix_buffer;
	VkDeviceMemory uniform_memory;
	VkDescriptorSet desc_set;
	VkDescriptorPool desc_pool;
	vw_pipeline_t pipeline;
} vw_instance_t;*/

typedef struct {
	VkDeviceMemory vertex_buffer_memory;
	VkBuffer vertex_input_buffer;
	uint32_t vertice_count;
} vw_shape_t;

typedef struct {
	VkImage mappable_image;
	VkDeviceMemory mappable_memory;
	VkImage image;
	VkDeviceMemory memory;
	VkSampler sampler;
	VkImageView view;
	uint32_t w, h;
	uint32_t pitch;
	uint8_t staged;
} vw_texture_t;
