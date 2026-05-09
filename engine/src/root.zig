const std = @import("std");
pub const c = @import("c");
const math = @import("math.zig");

pub const Vertex = extern struct {
    x: f32,
    y: f32,
};

pub const Engine = struct {
    const vertices = [_]Vertex{
        .{ .x = 100, .y = 100 }, // 0: bottom-left
        .{ .x = 300, .y = 100 }, // 1: bottom-right
        .{ .x = 300, .y = 300 }, // 2: top-right
        .{ .x = 100, .y = 300 }, // 3: top-left
    };
    const indices = [_]u16{ 0, 1, 2, 2, 3, 0 };

    w: *c.GLFWwindow,

    instance: c.WGPUInstance,
    adapter: c.WGPUAdapter,
    device: c.WGPUDevice,
    queue: c.WGPUQueue,
    surface: c.WGPUSurface,

    format: c.WGPUTextureFormat,
    shader: c.WGPUShaderModule,
    pipeline_layout: c.WGPUPipelineLayout,
    pipeline: c.WGPURenderPipeline,

    vertex_buffer: c.WGPUBuffer,
    index_buffer: c.WGPUBuffer,
    uniform_buffer: c.WGPUBuffer,

    bind_group_layout: c.WGPUBindGroupLayout,
    bind_group: c.WGPUBindGroup,

    pub fn init() ?Engine {
        if (c.glfwInit() == 0) {
            return null;
        }

        const w = c.glfwCreateWindow(720, 480, "Title", null, null) orelse return null;

        const instance = c.wgpuCreateInstance(null) orelse return null;

        // --- ADAPTER START ---
        var adapter: ?c.WGPUAdapter = null;
        _ = c.wgpuInstanceRequestAdapter(instance, null, .{
            .mode = c.WGPUCallbackMode_AllowProcessEvents,
            .callback = onAdapter,
            .userdata1 = &adapter,
        });
        while (adapter == null) c.wgpuInstanceProcessEvents(instance);
        // --- ADAPTER END ---

        // --- DEVICE START ---

        var device: ?c.WGPUDevice = null;

        var d_desc = std.mem.zeroes(c.WGPUDeviceDescriptor);
        d_desc.uncapturedErrorCallbackInfo.callback = onUncapturedError;
        _ = c.wgpuAdapterRequestDevice(adapter.?, &d_desc, .{
            .mode = c.WGPUCallbackMode_AllowProcessEvents,
            .callback = onDevice,
            .userdata1 = &device,
        });
        while (device == null) c.wgpuInstanceProcessEvents(instance);

        const queue = c.wgpuDeviceGetQueue(device.?);

        // --- DEVICE END ---

        // --- SURFACE START ---
        const surface = switch (c.glfwGetPlatform()) {
            c.GLFW_PLATFORM_X11 => blk: {
                var desc = std.mem.zeroes(c.WGPUSurfaceDescriptor);

                var src: c.WGPUSurfaceSourceXlibWindow = .{
                    .chain = .{ .next = null, .sType = c.WGPUSType_SurfaceSourceXlibWindow },
                    .display = c.glfwGetX11Display(),
                    .window = c.glfwGetX11Window(w),
                };

                desc.nextInChain = @ptrCast(&src);
                break :blk c.wgpuInstanceCreateSurface(instance, &desc);
            },
            c.GLFW_PLATFORM_WAYLAND => blk: {
                var desc = std.mem.zeroes(c.WGPUSurfaceDescriptor);

                var src: c.WGPUSurfaceSourceWaylandSurface = .{
                    .chain = .{ .next = null, .sType = c.WGPUSType_SurfaceSourceWaylandSurface },
                    .display = c.glfwGetWaylandDisplay(),
                    .surface = c.glfwGetWaylandWindow(w),
                };
                desc.nextInChain = @ptrCast(&src);
                break :blk c.wgpuInstanceCreateSurface(instance, &desc);
            },
            else => return null,
        };

        var fb_w: c_int = 0;
        var fb_h: c_int = 0;
        c.glfwGetFramebufferSize(w, &fb_w, &fb_h);

        var caps: c.WGPUSurfaceCapabilities = std.mem.zeroes(c.WGPUSurfaceCapabilities);
        _ = c.wgpuSurfaceGetCapabilities(surface, adapter.?, &caps);

        const format = caps.formats[0];
        // --- SURFACE END ---

        // --- UNIFORMS START ---
        const bgl_entry: c.WGPUBindGroupLayoutEntry = .{
            .nextInChain = null,
            .binding = 0,
            .visibility = c.WGPUShaderStage_Vertex,
            .buffer = .{
                .nextInChain = null,
                .type = c.WGPUBufferBindingType_Uniform,
                .hasDynamicOffset = 0,
                .minBindingSize = @sizeOf(math.Mat4),
            },
            .sampler = std.mem.zeroes(c.WGPUSamplerBindingLayout),
            .texture = std.mem.zeroes(c.WGPUTextureBindingLayout),
            .storageTexture = std.mem.zeroes(c.WGPUStorageTextureBindingLayout),
        };

        const bgl_desc: c.WGPUBindGroupLayoutDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .entryCount = 1,
            .entries = &bgl_entry,
        };

        const bind_group_layout = c.wgpuDeviceCreateBindGroupLayout(device.?, &bgl_desc) orelse return null;
        const pl_desc: c.WGPUPipelineLayoutDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .bindGroupLayoutCount = 1,
            .bindGroupLayouts = &bind_group_layout,
        };
        const pipeline_layout = c.wgpuDeviceCreatePipelineLayout(device.?, &pl_desc) orelse return null;

        const uniform_desc: c.WGPUBufferDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .usage = c.WGPUBufferUsage_Uniform | c.WGPUBufferUsage_CopyDst,
            .size = @sizeOf(math.Mat4),
            .mappedAtCreation = 0,
        };

        const uniform_buffer = c.wgpuDeviceCreateBuffer(device.?, &uniform_desc) orelse return null;

        const bg_entry: c.WGPUBindGroupEntry = .{
            .nextInChain = null,
            .binding = 0,
            .buffer = uniform_buffer,
            .offset = 0,
            .size = @sizeOf(math.Mat4),
            .sampler = null,
            .textureView = null,
        };

        const bg_desc: c.WGPUBindGroupDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .layout = bind_group_layout,
            .entryCount = 1,
            .entries = &bg_entry,
        };

        const bind_group = c.wgpuDeviceCreateBindGroup(device.?, &bg_desc) orelse return null;

        // --- UNIFORMS END ---

        // --- PIPELINE START ---

        const shader_src = @embedFile("shaders/triangle.wgsl");
        var wgsl: c.WGPUShaderSourceWGSL = .{
            .chain = .{ .next = null, .sType = c.WGPUSType_ShaderSourceWGSL },
            .code = .{ .data = shader_src.ptr, .length = shader_src.len },
        };

        var shader_desc = std.mem.zeroes(c.WGPUShaderModuleDescriptor);
        shader_desc.nextInChain = @ptrCast(&wgsl);

        const shader = c.wgpuDeviceCreateShaderModule(device.?, &shader_desc);

        const color_target: c.WGPUColorTargetState = .{
            .nextInChain = null,
            .format = format,
            .blend = null,
            .writeMask = c.WGPUColorWriteMask_All,
        };

        const fragment: c.WGPUFragmentState = .{
            .nextInChain = null,
            .module = shader,
            .entryPoint = .{ .data = "fs", .length = 2 },
            .constantCount = 0,
            .constants = null,
            .targetCount = 1,
            .targets = &color_target,
        };

        var primitive = std.mem.zeroes(c.WGPUPrimitiveState);
        primitive.topology = c.WGPUPrimitiveTopology_TriangleList;
        primitive.frontFace = c.WGPUFrontFace_CCW;
        primitive.cullMode = c.WGPUCullMode_None;

        var multisample = std.mem.zeroes(c.WGPUMultisampleState);
        multisample.count = 1;
        multisample.mask = 0xFFFFFF;

        const vertex_attr: c.WGPUVertexAttribute = .{
            .format = c.WGPUVertexFormat_Float32x2,
            .offset = 0,
            .shaderLocation = 0,
        };

        const vertex_layout: c.WGPUVertexBufferLayout = .{
            .arrayStride = @sizeOf(Vertex),
            .stepMode = c.WGPUVertexStepMode_Vertex,
            .attributeCount = 1,
            .attributes = &vertex_attr,
        };

        const pipeline_desc: c.WGPURenderPipelineDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .layout = pipeline_layout,
            .vertex = .{
                .nextInChain = null,
                .module = shader,
                .entryPoint = .{ .data = "vs", .length = 2 },
                .constantCount = 0,
                .constants = null,
                .bufferCount = 1,
                .buffers = &vertex_layout,
            },
            .primitive = primitive,
            .depthStencil = null,
            .multisample = multisample,
            .fragment = &fragment,
        };

        const pipeline = c.wgpuDeviceCreateRenderPipeline(device.?, &pipeline_desc) orelse return null;
        // --- PIPELINE END ---

        const buffer_size: u64 = @sizeOf(Vertex) * vertices.len;
        const buffer_desc: c.WGPUBufferDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .usage = c.WGPUBufferUsage_Vertex | c.WGPUBufferUsage_CopyDst,
            .size = buffer_size,
            .mappedAtCreation = 0,
        };
        const vertex_buffer = c.wgpuDeviceCreateBuffer(device.?, &buffer_desc) orelse return null;
        c.wgpuQueueWriteBuffer(queue, vertex_buffer, 0, &vertices, buffer_size);

        const index_size: u64 = @sizeOf(u16) * indices.len;
        const index_desc: c.WGPUBufferDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .usage = c.WGPUBufferUsage_Index | c.WGPUBufferUsage_CopyDst,
            .size = index_size,
            .mappedAtCreation = 0,
        };
        const index_buffer = c.wgpuDeviceCreateBuffer(device.?, &index_desc) orelse return null;
        c.wgpuQueueWriteBuffer(queue, index_buffer, 0, &indices, index_size);

        var engine: Engine = .{
            .w = w,
            .instance = instance,
            .adapter = adapter.?,
            .device = device.?,
            .queue = queue,
            .surface = surface,

            .format = format,
            .shader = shader,
            .pipeline = pipeline,
            .pipeline_layout = pipeline_layout,

            .vertex_buffer = vertex_buffer,
            .index_buffer = index_buffer,
            .bind_group_layout = bind_group_layout,
            .bind_group = bind_group,

            .uniform_buffer = uniform_buffer,
        };

        engine.configureSurface(fb_w, fb_h);

        return engine;
    }

    pub fn deinit(self: *Engine) void {
        c.wgpuBindGroupRelease(self.bind_group);
        c.wgpuBindGroupLayoutRelease(self.bind_group_layout);

        c.wgpuBufferRelease(self.uniform_buffer);
        c.wgpuBufferRelease(self.index_buffer);
        c.wgpuBufferRelease(self.vertex_buffer);

        c.wgpuPipelineLayoutRelease(self.pipeline_layout);
        c.wgpuRenderPipelineRelease(self.pipeline);
        c.wgpuShaderModuleRelease(self.shader);
        c.wgpuSurfaceUnconfigure(self.surface);
        c.wgpuSurfaceRelease(self.surface);
        c.wgpuQueueRelease(self.queue);
        c.wgpuDeviceRelease(self.device);
        c.wgpuAdapterRelease(self.adapter);
        c.wgpuInstanceRelease(self.instance);

        c.glfwDestroyWindow(self.w);
        c.glfwTerminate();
    }

    pub fn run(self: *Engine) void {
        c.glfwSetWindowUserPointer(self.w, self);
        _ = c.glfwSetFramebufferSizeCallback(self.w, onFramebufferSize);

        while (c.glfwWindowShouldClose(self.w) == 0) {
            c.glfwPollEvents();
            self.render();
        }
    }

    pub fn render(self: *Engine) void {
        var st: c.WGPUSurfaceTexture = std.mem.zeroes(c.WGPUSurfaceTexture);
        c.wgpuSurfaceGetCurrentTexture(self.surface, &st);

        if (st.status != c.WGPUSurfaceGetCurrentTextureStatus_SuccessOptimal and
            st.status != c.WGPUSurfaceGetCurrentTextureStatus_SuccessSuboptimal)
        {
            return;
        }

        const view = c.wgpuTextureCreateView(st.texture, null) orelse return;
        defer c.wgpuTextureViewRelease(view);

        const encoder = c.wgpuDeviceCreateCommandEncoder(self.device, null) orelse return;
        defer c.wgpuCommandEncoderRelease(encoder);

        const color_attachment: c.WGPURenderPassColorAttachment = .{
            .nextInChain = null,
            .view = view,
            .depthSlice = c.WGPU_DEPTH_SLICE_UNDEFINED,
            .resolveTarget = null,
            .loadOp = c.WGPULoadOp_Clear,
            .storeOp = c.WGPUStoreOp_Store,
            .clearValue = .{ .r = 0.1, .g = 0.2, .b = 0.4, .a = 1.0 },
        };

        const pass_desc: c.WGPURenderPassDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .colorAttachmentCount = 1,
            .colorAttachments = &color_attachment,
            .depthStencilAttachment = null,
            .occlusionQuerySet = null,
            .timestampWrites = null,
        };

        const pass = c.wgpuCommandEncoderBeginRenderPass(encoder, &pass_desc);

        c.wgpuRenderPassEncoderSetPipeline(pass, self.pipeline);
        c.wgpuRenderPassEncoderSetBindGroup(pass, 0, self.bind_group, 0, null);
        c.wgpuRenderPassEncoderSetVertexBuffer(pass, 0, self.vertex_buffer, 0, @sizeOf(Vertex) * vertices.len);
        c.wgpuRenderPassEncoderSetIndexBuffer(pass, self.index_buffer, c.WGPUIndexFormat_Uint16, 0, @sizeOf(u16) * indices.len);
        c.wgpuRenderPassEncoderDrawIndexed(pass, indices.len, 1, 0, 0, 0);

        c.wgpuRenderPassEncoderEnd(pass);
        c.wgpuRenderPassEncoderRelease(pass);

        const cmd = c.wgpuCommandEncoderFinish(encoder, null) orelse return;
        defer c.wgpuCommandBufferRelease(cmd);

        c.wgpuQueueSubmit(self.queue, 1, &cmd);
        _ = c.wgpuSurfacePresent(self.surface);

        c.wgpuTextureRelease(st.texture);
    }

    fn onAdapter(
        status: c.WGPURequestAdapterStatus,
        adapter: c.WGPUAdapter,
        message: c.WGPUStringView,
        userdata1: ?*anyopaque,
        userdata2: ?*anyopaque,
    ) callconv(.c) void {
        _ = message;
        _ = userdata2;

        if (status != c.WGPURequestAdapterStatus_Success) return;
        const out: *?c.WGPUAdapter = @ptrCast(@alignCast(userdata1));
        out.* = adapter;
    }

    fn onDevice(
        status: c.WGPURequestDeviceStatus,
        device: c.WGPUDevice,
        message: c.WGPUStringView,
        userdata1: ?*anyopaque,
        userdata2: ?*anyopaque,
    ) callconv(.c) void {
        _ = message;
        _ = userdata2;

        if (status != c.WGPURequestDeviceStatus_Success) return;

        const out: *?c.WGPUDevice = @ptrCast(@alignCast(userdata1));
        out.* = device;
    }

    fn onUncapturedError(
        device: [*c]const c.WGPUDevice,
        err_type: c.WGPUErrorType,
        message: c.WGPUStringView,
        userdata1: ?*anyopaque,
        userdata2: ?*anyopaque,
    ) callconv(.c) void {
        _ = device;
        _ = userdata1;
        _ = userdata2;

        const text = if (message.data != null and message.length > 0)
            message.data[0..message.length]
        else
            "(no message)";

        std.debug.print("wgpu error (type={d}): {s}\n", .{ err_type, text });
    }

    fn onFramebufferSize(window: ?*c.GLFWwindow, w: c_int, h: c_int) callconv(.c) void {
        const self: *Engine = @ptrCast(@alignCast(
            c.glfwGetWindowUserPointer(window).?,
        ));

        self.configureSurface(w, h);
    }

    pub fn configureSurface(self: *Engine, w: c_int, h: c_int) void {
        if (w <= 0 or h <= 0) return;

        var caps: c.WGPUSurfaceCapabilities = std.mem.zeroes(c.WGPUSurfaceCapabilities);
        _ = c.wgpuSurfaceGetCapabilities(self.surface, self.adapter, &caps);
        defer c.wgpuSurfaceCapabilitiesFreeMembers(caps);

        // Bottom is greater than top here because of top-to-bottom standard
        // So 0x0 becomes the top-left corner
        // With higher X we go to right
        // With higher Y we go to bottom
        const proj = math.Mat4.ortho(0, @floatFromInt(w), @floatFromInt(h), 0, -1, 1);
        c.wgpuQueueWriteBuffer(self.queue, self.uniform_buffer, 0, &proj, @sizeOf(math.Mat4));

        const config: c.WGPUSurfaceConfiguration = .{
            .nextInChain = null,
            .device = self.device,
            .format = self.format,
            .usage = c.WGPUTextureUsage_RenderAttachment,
            .viewFormatCount = 0,
            .viewFormats = null,
            .alphaMode = caps.alphaModes[0],
            .width = @intCast(w),
            .height = @intCast(h),
            .presentMode = c.WGPUPresentMode_Fifo,
        };
        c.wgpuSurfaceConfigure(self.surface, &config);
    }
};

test "compile-check Engine" {
    var e = Engine.init() orelse return error.InitFailed;
    defer e.deinit();

    e.run();
}
