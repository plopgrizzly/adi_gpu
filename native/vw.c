/**
 * Aldaron's Device Interface - "vw.c"
 * Copyright 2017 (c) Jeron Lau - Licensed under the GNU GENERAL PUBLIC LICENSE
 *
 * This file is based off of LunarG's SDK example:
 * https://www.lunarg.com/vulkan-sdk/, and this tutorial by José Henriques:
 * http://jhenriques.net/vulkan_shaders.html,
**/

#include "vw.h"

static inline void vw_vulkan_error(const char *msg, VkResult result) {
	if(result != VK_SUCCESS) {
		printf("abort on error %d!", result);
		puts(msg);
		abort();
	}
}

// TODO: Delete
static uint32_t memory_type_from_properties(const vw_t* vulkan, uint32_t typeBits,
	VkFlags reqs_mask)
{
	VkPhysicalDeviceMemoryProperties props;
	vkGetPhysicalDeviceMemoryProperties(vulkan->gpu, &props);
	for (uint32_t i = 0; i < props.memoryTypeCount; i++) {
		// Memory type req's matches vkGetImageMemoryRequirements()?
		if ((typeBits & 1) == 1) {
			// Is requirements_mask's requirements fullfilled?
			if ((props.memoryTypes[i].propertyFlags & reqs_mask) ==
				reqs_mask)
			{
				return i;
			}
		}
		// Check next bit from vkGetImageMemoryRequirements().
		typeBits >>= 1;
	}
	// Nothing works ... fallback to 0 and hope nothing bad happens.
	puts("ALDARON WARNING: Couldn't find suitable memory type.");
	return 0;
}

// TODO: REMOVE
void vw_vulkan_txuniform(const vw_t* vulkan, vw_instance_t* instance,
	const vw_texture_t* tx, uint8_t tex_count)
{
	const int NUM_WRITES = !!tex_count;
	VkDescriptorImageInfo tex_desc;
	if(tex_count) {
		tex_desc = (VkDescriptorImageInfo) {
			.sampler = tx->sampler,
			.imageView = tx->view,
			.imageLayout = VK_IMAGE_LAYOUT_GENERAL,
		};
	}

	VkWriteDescriptorSet writes[1 + NUM_WRITES]; // 2
	memset(&writes, 0, sizeof(writes));

	VkDescriptorBufferInfo buffer_info = {
		.buffer = instance->matrix_buffer,
		.offset = 0,
		.range = sizeof(float) * 4, // 16
	};
	writes[0] = (VkWriteDescriptorSet) {
		.sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
		.dstSet = instance->desc_set,
		.descriptorCount = 1,
		.descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
		.pBufferInfo = &buffer_info,
	};

	if(NUM_WRITES) {
		writes[1] = (VkWriteDescriptorSet) {
			.sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
			.dstSet = instance->desc_set,
			.dstBinding = 1,
			.descriptorCount = tex_count,
			.descriptorType = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
			.pImageInfo = &tex_desc,
		};
	}

	vkUpdateDescriptorSets(vulkan->device, 1 + NUM_WRITES, writes, 0, NULL);
}

// Called From Rust FFI
vw_instance_t vw_vulkan_uniforms(const vw_t* vulkan, vw_pipeline_t pipeline,
	const vw_texture_t* tx, uint8_t tex_count)
{
	vw_instance_t instance;

	// Buffers
	VkBufferCreateInfo uniform_buffer_ci = {
		.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
		.size = sizeof(float) * 16, // mat4
		.usage = VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT,
		.sharingMode = VK_SHARING_MODE_EXCLUSIVE,
		.queueFamilyIndexCount = 0,
		.pQueueFamilyIndices = NULL,
	};
	vw_vulkan_error("Failed to create matrix buffer.", vkCreateBuffer(
		vulkan->device, &uniform_buffer_ci, NULL,
		&instance.matrix_buffer));

	// Descriptor Pool
	const VkDescriptorPoolSize type_counts[2] = {
		[0] = {
			 .type = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
			 .descriptorCount = 1,
		},
		[1] = {
			 .type = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
			 .descriptorCount = tex_count, // Texture count
		},
	};
	const VkDescriptorPoolCreateInfo descriptor_pool = {
		.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
		.pNext = NULL,
		.maxSets = 1,
		.poolSizeCount = 1 + tex_count,
		.pPoolSizes = &type_counts[0],
	};
	vw_vulkan_error("Failed to create descriptor pool.",
		vkCreateDescriptorPool(vulkan->device, &descriptor_pool, NULL,
			&instance.desc_pool));

	VkDescriptorSetAllocateInfo alloc_info = {
		.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
		.pNext = NULL,
		.descriptorPool = instance.desc_pool,
		.descriptorSetCount = 1,
		.pSetLayouts = &pipeline.descsetlayout
	};
	vw_vulkan_error("Failed to allocate descriptor sets.",
		vkAllocateDescriptorSets(vulkan->device, &alloc_info,
			&instance.desc_set));

// {
	instance.uniform_memory = 0;

	// Allocate memory for uniform buffer.
	VkMemoryRequirements mem_reqs;
	vkGetBufferMemoryRequirements(vulkan->device, instance.matrix_buffer,
		&mem_reqs);
	VkMemoryAllocateInfo buffer_ai = {
		.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
		.pNext = NULL,
		.allocationSize = mem_reqs.size,
		.memoryTypeIndex = memory_type_from_properties(vulkan,
			mem_reqs.memoryTypeBits,
			VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT |
			VK_MEMORY_PROPERTY_HOST_COHERENT_BIT),
	};

	vw_vulkan_error("Failed to allocate uniform memory.", vkAllocateMemory(
		vulkan->device, &buffer_ai, NULL, &instance.uniform_memory));
	vkBindBufferMemory(vulkan->device, instance.matrix_buffer,
		instance.uniform_memory, 0);
// }
	vw_vulkan_txuniform(vulkan, &instance, tx, tex_count);

	instance.pipeline = pipeline;
	return instance;
}

// TODO: Remove after Windows port works.
float* test_map(VkDevice device, VkDeviceMemory vertex_buffer_memory, uint64_t wholesize) {
	void* mapped = NULL;
	vw_vulkan_error("Failed to test map buffer memory.", vkMapMemory(
		device, vertex_buffer_memory, 0, wholesize, 0,
		&mapped));
	return mapped;
}

void vw_vulkan_animate(vw_t* vulkan, vw_texture_t* tx, uint32_t w, uint32_t h,
	const uint8_t* p)
{
	void *data;
	vw_vulkan_error("map memory", vkMapMemory(vulkan->device,
		tx->mappable_memory, 0, tx->size, 0, &data));

	for (uint32_t y = 0; y < h; y++) {
		uint8_t *rowPtr = data;
		for (uint32_t x = 0; x < w; x++) {
			memcpy(rowPtr, &p[((y * w) + x) * 4], 4);
			rowPtr += 4;
		}
		data += tx->pitch;
	}

	vkUnmapMemory(vulkan->device, tx->mappable_memory);

	if (!tx->staged) {
		// Use a linear tiled image for the texture, is supported
		tx->image = tx->mappable_image;
		tx->memory = tx->mappable_memory;
	} else {
		// Use optimal tiled image - create from linear tiled image
		VkMemoryRequirements mem_reqs;
		vkGetImageMemoryRequirements(vulkan->device, 0,
			&mem_reqs);

		VkImageCopy copy_region = {
			.srcSubresource = {
				.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT,
				.mipLevel = 0,
				.baseArrayLayer = 0,
				.layerCount = 1,
			},
			.srcOffset = { .x = 0, .y = 0, .z = 0 },
			.dstSubresource = {
				.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT,
				.mipLevel = 0,
				.baseArrayLayer = 0,
				.layerCount = 1,
			},
			.dstOffset = { .x = 0, .y = 0, .z = 0 },
			.extent = { .width = w, .height = h, .depth = 1 },
		};

		// Copy data from linear image to optimal image.
		vkCmdCopyImage(vulkan->command_buffer, tx->mappable_image,
			VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL, tx->image,
			VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &copy_region);
	}
}

vw_texture_t vw_vulkan_texture(vw_t* vulkan, uint32_t w, uint32_t h,
	const uint8_t* p)
{
	vw_texture_t texture;
	VkFormatProperties formatProps;
	VkMemoryRequirements mem_reqs;

	// Use staging image if linear tiled image isn't supported 
	vkGetPhysicalDeviceFormatProperties(vulkan->gpu,
		VK_FORMAT_B8G8R8A8_UNORM, &formatProps);
	texture.staged = (!(formatProps.linearTilingFeatures &
		VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT)) ? 1 : 0;

	VkImageCreateInfo image_create_info = {
		.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
		.pNext = NULL,
		.imageType = VK_IMAGE_TYPE_2D,
		.format = VK_FORMAT_B8G8R8A8_SRGB,
		.extent = { .width = w, .height = h, .depth = 1 },
		.mipLevels = 1,
		.arrayLayers = 1,
		.samples = VK_SAMPLE_COUNT_1_BIT,
		.tiling = VK_IMAGE_TILING_LINEAR,
		.initialLayout = VK_IMAGE_LAYOUT_PREINITIALIZED,
		.usage = texture.staged ? VK_IMAGE_USAGE_TRANSFER_SRC_BIT
			: VK_IMAGE_USAGE_SAMPLED_BIT,
		.queueFamilyIndexCount = 0,
		.pQueueFamilyIndices = NULL,
		.sharingMode = VK_SHARING_MODE_EXCLUSIVE,
		.flags = 0,
	};

	// Create linear tiled image
	vw_vulkan_error("create image", vkCreateImage(vulkan->device,
		&image_create_info, NULL, &texture.mappable_image));
	vkGetImageMemoryRequirements(vulkan->device, texture.mappable_image, &mem_reqs);

	VkMemoryAllocateInfo mem_alloc = {
		.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
		.pNext = NULL,
		.allocationSize = mem_reqs.size,
		.memoryTypeIndex = memory_type_from_properties(vulkan,
			mem_reqs.memoryTypeBits,
			VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT |
			VK_MEMORY_PROPERTY_HOST_COHERENT_BIT),
	};

	vw_vulkan_error("allocate memory", vkAllocateMemory(vulkan->device,
		&mem_alloc, NULL, &(texture.mappable_memory)));
	vw_vulkan_error("bind memory", vkBindImageMemory(vulkan->device,
		texture.mappable_image, texture.mappable_memory, 0));

	const VkImageSubresource subres = {
		.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT, .mipLevel = 0,
		.arrayLayer = 0,
	};

	VkSubresourceLayout layout;
	vkGetImageSubresourceLayout(vulkan->device, texture.mappable_image,
		&subres, &layout);

	texture.size = mem_reqs.size;
	texture.pitch = layout.rowPitch;

	const VkImageCreateInfo image_create_2info = {
		.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
		.pNext = NULL,
		.imageType = VK_IMAGE_TYPE_2D,
		.format = VK_FORMAT_B8G8R8A8_UNORM,
		.extent = { .width = w, .height = h, .depth = 1 },
		.mipLevels = 1,
		.arrayLayers = 1,
		.samples = VK_SAMPLE_COUNT_1_BIT,
		.tiling = VK_IMAGE_TILING_OPTIMAL,
		.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED,
		.usage = VK_IMAGE_USAGE_TRANSFER_DST_BIT |
			VK_IMAGE_USAGE_SAMPLED_BIT,
		.queueFamilyIndexCount = 0,
		.pQueueFamilyIndices = NULL,
		.sharingMode = VK_SHARING_MODE_EXCLUSIVE,
		.flags = 0,
	};

	vw_vulkan_error("bind memory", vkCreateImage(vulkan->device,
		&image_create_2info, NULL, &texture.image));

	if (texture.staged) {
		const VkMemoryAllocateInfo mem_alloc = {
			.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
			.pNext = NULL,
			.allocationSize = mem_reqs.size,
			.memoryTypeIndex = memory_type_from_properties(vulkan,
				mem_reqs.memoryTypeBits, 0),
		};

		vw_vulkan_error("allocate memory", vkAllocateMemory(
			vulkan->device, &mem_alloc, NULL, &texture.memory));
		vw_vulkan_error("bind image memory", vkBindImageMemory(
			vulkan->device, texture.image, texture.memory, 0));
	}

	vw_vulkan_animate(vulkan, &texture, w, h, p);

	VkSamplerCreateInfo samplerCreateInfo = {
		.sType = VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
		.magFilter = VK_FILTER_NEAREST,
		.minFilter = VK_FILTER_NEAREST,
		.mipmapMode = VK_SAMPLER_MIPMAP_MODE_NEAREST,
		.addressModeU = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
		.addressModeV = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
		.addressModeW = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
		.mipLodBias = 0.0,
		.anisotropyEnable = VK_FALSE,
		.maxAnisotropy = 0,
		.compareEnable = VK_FALSE,
		.compareOp = VK_COMPARE_OP_NEVER,
		.minLod = 0.0,
		.maxLod = 0.0,
		.borderColor = VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
	};

	vw_vulkan_error("create sampler", vkCreateSampler(vulkan->device,
		&samplerCreateInfo, NULL, &texture.sampler));

	VkImageViewCreateInfo view_info = {
		.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
		.pNext = NULL,
		.image = texture.image,
		.viewType = VK_IMAGE_VIEW_TYPE_2D,
		.format = VK_FORMAT_B8G8R8A8_SRGB,
		.components.r = VK_COMPONENT_SWIZZLE_R,
		.components.g = VK_COMPONENT_SWIZZLE_G,
		.components.b = VK_COMPONENT_SWIZZLE_B,
		.components.a = VK_COMPONENT_SWIZZLE_A,
		.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT,
		.subresourceRange.baseMipLevel = 0,
		.subresourceRange.levelCount = 1,
		.subresourceRange.baseArrayLayer = 0,
		.subresourceRange.layerCount = 1,
	};
	vw_vulkan_error("create image view", vkCreateImageView(vulkan->device,
		&view_info, NULL, &texture.view));
	return texture;
}

void vw_vulkan_draw_begin(vw_t* vulkan, float r, float g, float b) {
	/*VkSemaphoreCreateInfo semaphore_ci = {
		VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO, 0, 0
	};
	vkCreateSemaphore(vulkan->device, &semaphore_ci, NULL,
		&vulkan->presenting_complete_sem);

	VkResult result = vkAcquireNextImageKHR(
		vulkan->device, vulkan->swapchain, UINT64_MAX,
		vulkan->presenting_complete_sem, VK_NULL_HANDLE,
		&vulkan->next_image_index);

	if (result == VK_ERROR_OUT_OF_DATE_KHR) {
		vkDestroySemaphore(vulkan->device,
			vulkan->presenting_complete_sem, NULL);

		vkCreateSemaphore(vulkan->device, &semaphore_ci, NULL,
			&vulkan->presenting_complete_sem);

		if( vkAcquireNextImageKHR(
			vulkan->device, vulkan->swapchain, UINT64_MAX,
			vulkan->presenting_complete_sem, VK_NULL_HANDLE,
			&vulkan->next_image_index) != VK_SUCCESS)
		{
			printf("vkAcquireNextImageKHR Failed %d!\n", result);
			exit(-1);
		}
	} else if (result != VK_SUCCESS) {
		printf("vkAcquireNextImageKHR Failed %d!\n", result);
		exit(-1);
	}

	vkCreateSemaphore(vulkan->device, &semaphore_ci, NULL,
		&vulkan->rendering_complete_sem);*/

	VkCommandBufferBeginInfo beginInfo = {
		.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
		.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
	};

	vkBeginCommandBuffer(vulkan->command_buffer, &beginInfo);

	VkImageMemoryBarrier layoutTransitionBarrier = {
		.sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
		.srcAccessMask = VK_ACCESS_MEMORY_READ_BIT,
		.dstAccessMask = VK_ACCESS_COLOR_ATTACHMENT_READ_BIT | VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
		.oldLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
		.newLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
		.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
		.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
		.image = vulkan->present_images[ vulkan->next_image_index ],
	};

	VkImageSubresourceRange resourceRange = { VK_IMAGE_ASPECT_COLOR_BIT, 0, 1, 0, 1 };
	layoutTransitionBarrier.subresourceRange = resourceRange;

	vkCmdPipelineBarrier(vulkan->command_buffer,
		VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT, 
		VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT, 
		0, 0, NULL, 0, NULL, 1, &layoutTransitionBarrier);

	// activate render pass:
	VkClearValue clearValue[2] = {
		[0] = { .color.float32 = { r, g, b, 1.0f } },
		[1] = { .depthStencil = (VkClearDepthStencilValue) { 1.0, 0 } },
	};
	VkRenderPassBeginInfo renderPassBeginInfo = {
		.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
		.renderPass = vulkan->render_pass,
		.framebuffer = vulkan->frame_buffers[vulkan->next_image_index],
		.renderArea = {
			.offset = { .x = 0, .y = 0 },
			.extent = {
				.width = vulkan->width,
				.height = vulkan->height
			},
		},
		.clearValueCount = 2,
		.pClearValues = clearValue,
	};
	vkCmdBeginRenderPass(vulkan->command_buffer, &renderPassBeginInfo,
		VK_SUBPASS_CONTENTS_INLINE);

	// take care of dynamic state:
	VkViewport viewport = { 0, 0, vulkan->width, vulkan->height, 0, 1 };
	vkCmdSetViewport(vulkan->command_buffer, 0, 1, &viewport);

	VkRect2D scissor = {
		.offset = { 0, 0 }, .extent = { vulkan->width, vulkan->height },
	};
	vkCmdSetScissor(vulkan->command_buffer, 0, 1, &scissor);
}

void vw_vulkan_draw_update(vw_t* vulkan) {
	vkCmdEndRenderPass(vulkan->command_buffer);
	// change layout back to VK_IMAGE_LAYOUT_PRESENT_SRC_KHR
	VkImageMemoryBarrier prePresentBarrier = {
		.sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
		.srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
		.dstAccessMask = VK_ACCESS_MEMORY_READ_BIT,
		.oldLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
		.newLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
		.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
		.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
		.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT,
		.subresourceRange.baseMipLevel = 0,
		.subresourceRange.levelCount = 1,
		.subresourceRange.baseArrayLayer = 0,
		.subresourceRange.layerCount = 1,
		.image = vulkan->present_images[vulkan->next_image_index],
	};
	vkCmdPipelineBarrier(vulkan->command_buffer,
		VK_PIPELINE_STAGE_ALL_COMMANDS_BIT,
		VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, 0, 0, NULL, 0, NULL, 1,
		&prePresentBarrier);

	vkEndCommandBuffer(vulkan->command_buffer);
	// present:
	VkFence render_fence;
	VkFenceCreateInfo fenceCreateInfo = {
		.sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
		.pNext = NULL,
		.flags = 0,
	};
	vkCreateFence(vulkan->device, &fenceCreateInfo, NULL, &render_fence);
	VkPipelineStageFlags waitStageMash=VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT;
	VkSubmitInfo submitInfo = {
		.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO,
		.waitSemaphoreCount = 1,
		.pWaitSemaphores = &vulkan->presenting_complete_sem,
		.pWaitDstStageMask = &waitStageMash,
		.commandBufferCount = 1,
		.pCommandBuffers = &vulkan->command_buffer,
		.signalSemaphoreCount = 1,
		.pSignalSemaphores = &vulkan->rendering_complete_sem,
	};
	vkQueueSubmit(vulkan->present_queue, 1, &submitInfo, render_fence);
	vkWaitForFences(vulkan->device, 1, &render_fence, VK_TRUE, UINT64_MAX);
	vkDestroyFence(vulkan->device, render_fence, NULL);
	VkPresentInfoKHR present_info = {
		.sType = VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
		.pNext = NULL,
		.waitSemaphoreCount = 1,
		.pWaitSemaphores = &vulkan->rendering_complete_sem,
		.swapchainCount = 1,
		.pSwapchains = &vulkan->swapchain,
		.pImageIndices = &vulkan->next_image_index,
		.pResults = NULL,
	};
	vkQueuePresentKHR(vulkan->present_queue, &present_info);
	vkDestroySemaphore(vulkan->device, vulkan->presenting_complete_sem, NULL);
	vkDestroySemaphore(vulkan->device, vulkan->rendering_complete_sem, NULL);
	vkDeviceWaitIdle(vulkan->device);
}
