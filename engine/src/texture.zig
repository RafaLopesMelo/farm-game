pub const c = @import("c");
const math = @import("math.zig");

pub const Texture = struct {
    texture: c.WGPUTexture,
    view: c.WGPUTextureView,
    sampler: c.WGPUSampler,
    bind_group: c.WGPUBindGroup,
    width: u32,
    height: u32,

    pub fn loadFromFile(
        device: c.WGPUDevice,
        queue: c.WGPUQueue,
        layout: c.WGPUBindGroupLayout,
        uniform_buffer: c.WGPUBuffer,
        path: [*:0]const u8,
    ) ?Texture {
        var w: c_int = 0;
        var h: c_int = 0;
        var ch: c_int = 0;
        const img = c.stbi_load(path, &w, &h, &ch, 4);
        defer c.stbi_image_free(img);

        const tex_desc: c.WGPUTextureDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .usage = c.WGPUTextureUsage_TextureBinding | c.WGPUTextureUsage_CopyDst,
            .dimension = c.WGPUTextureDimension_2D,
            .size = .{ .height = @intCast(h), .width = @intCast(w), .depthOrArrayLayers = 1 },
            .format = c.WGPUTextureFormat_RGBA8Unorm,
            .mipLevelCount = 1,
            .sampleCount = 1,
            .viewFormatCount = 0,
            .viewFormats = null,
        };
        const texture = c.wgpuDeviceCreateTexture(device.?, &tex_desc) orelse return null;

        const sampler_desc: c.WGPUSamplerDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .addressModeU = c.WGPUAddressMode_ClampToEdge,
            .addressModeV = c.WGPUAddressMode_ClampToEdge,
            .addressModeW = c.WGPUAddressMode_ClampToEdge,
            .magFilter = c.WGPUFilterMode_Nearest,
            .minFilter = c.WGPUFilterMode_Nearest,
            .mipmapFilter = c.WGPUMipmapFilterMode_Nearest,
            .lodMinClamp = 0,
            .lodMaxClamp = 1,
            .compare = c.WGPUCompareFunction_Undefined,
            .maxAnisotropy = 1,
        };
        const sampler = c.wgpuDeviceCreateSampler(device.?, &sampler_desc) orelse return null;

        const view = c.wgpuTextureCreateView(texture, null) orelse return null;

        const bg_entries = [_]c.WGPUBindGroupEntry{
            .{
                .nextInChain = null,
                .binding = 0,
                .buffer = uniform_buffer,
                .offset = 0,
                .size = @sizeOf(math.Mat4),
                .sampler = null,
                .textureView = null,
            },
            .{
                .nextInChain = null,
                .binding = 1,
                .buffer = null,
                .offset = 0,
                .size = 0,
                .sampler = null,
                .textureView = view,
            },
            .{
                .nextInChain = null,
                .binding = 2,
                .buffer = null,
                .offset = 0,
                .size = 0,
                .sampler = sampler,
                .textureView = null,
            },
        };

        const bg_desc: c.WGPUBindGroupDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .layout = layout,
            .entryCount = bg_entries.len,
            .entries = &bg_entries,
        };

        const bind_group = c.wgpuDeviceCreateBindGroup(device, &bg_desc) orelse return null;

        const dest: c.WGPUTexelCopyTextureInfo = .{
            .texture = texture,
            .mipLevel = 0,
            .origin = .{ .x = 0, .y = 0, .z = 0 },
            .aspect = c.WGPUTextureAspect_All,
        };

        const buffer_layout: c.WGPUTexelCopyBufferLayout = .{
            .offset = 0,
            .bytesPerRow = @intCast(w * 4),
            .rowsPerImage = @intCast(h),
        };

        const extent: c.WGPUExtent3D = .{
            .width = @intCast(w),
            .height = @intCast(h),
            .depthOrArrayLayers = 1,
        };

        c.wgpuQueueWriteTexture(queue, &dest, img, @intCast(w * h * 4), &buffer_layout, &extent);

        return .{
            .texture = texture,
            .view = view,
            .sampler = sampler,
            .bind_group = bind_group,
            .width = @intCast(w),
            .height = @intCast(h),
        };
    }

    pub fn deinit(self: *Texture) void {
        c.wgpuTextureRelease(self.texture);
        c.wgpuTextureViewRelease(self.view);
        c.wgpuSamplerRelease(self.sampler);
        c.wgpuBindGroupRelease(self.bind_group);
    }
};
