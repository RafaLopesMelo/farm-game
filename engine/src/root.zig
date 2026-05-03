const std = @import("std");
pub const c = @import("c");

pub const Engine = struct {
    w: *c.GLFWwindow,

    instance: c.WGPUInstance,
    adapter: c.WGPUAdapter,
    device: c.WGPUDevice,
    queue: c.WGPUQueue,
    surface: c.WGPUSurface,

    pub fn init() ?Engine {
        if (c.glfwInit() == 0) {
            return null;
        }

        const w = c.glfwCreateWindow(720, 480, "Title", null, null) 
            orelse return null;

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
            .userdata1 = &device
        });
        while (device == null) c.wgpuInstanceProcessEvents(instance);

        // --- DEVICE END ---

        // --- SURFACE START ---
        const surface = switch(c.glfwGetPlatform()) {
            c.GLFW_PLATFORM_X11 => blk: {
                var desc = std.mem.zeroes(c.WGPUSurfaceDescriptor);

                var src: c.WGPUSurfaceSourceXlibWindow = .{
                    .chain = .{ .next = null, .sType = c.WGPUSType_SurfaceSourceXlibWindow },
                    .display = c.glfwGetX11Display(),
                    .window = c.glfwGetX11Window(w)
                };

                desc.nextInChain = @ptrCast(&src);
                break :blk c.wgpuInstanceCreateSurface(instance, &desc);
            },
            c.GLFW_PLATFORM_WAYLAND => blk: {
                var desc = std.mem.zeroes(c.WGPUSurfaceDescriptor);

                var src: c.WGPUSurfaceSourceWaylandSurface = .{
                    .chain = .{ .next = null, .sType = c.WGPUSType_SurfaceSourceWaylandSurface },
                    .display = c.glfwGetWaylandDisplay(),
                    .surface = c.glfwGetWaylandWindow(w)
                };
                desc.nextInChain = @ptrCast(&src);
                break :blk c.wgpuInstanceCreateSurface(instance, &desc);
            },
            else => return null
        };

        var fb_w: c_int = 0;
        var fb_h: c_int = 0;
        c.glfwGetFramebufferSize(w, &fb_w, &fb_h);

        var caps: c.WGPUSurfaceCapabilities = std.mem.zeroes(c.WGPUSurfaceCapabilities);
        _ = c.wgpuSurfaceGetCapabilities(surface, adapter.?, &caps);

        const config: c.WGPUSurfaceConfiguration = .{
            .nextInChain = null,
            .device = device.?,
            .format = caps.formats[0],
            .usage = c.WGPUTextureUsage_RenderAttachment,
            .viewFormatCount = 0,
            .viewFormats = null,
            .alphaMode = caps.alphaModes[0],
            .width = @intCast(fb_w),
            .height = @intCast(fb_h),
            .presentMode = c.WGPUPresentMode_Fifo
        };

        c.wgpuSurfaceConfigure(surface, &config);
        c.wgpuSurfaceCapabilitiesFreeMembers(caps);
        // --- SURFACE END ---

        return .{ 
            .w = w, 
            .instance = instance,
            .adapter = adapter.?,
            .device = device.?,
            .queue = c.wgpuDeviceGetQueue(device.?),
            .surface = surface
        };
    }

    pub fn deinit(self: *Engine) void {
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
        while (c.glfwWindowShouldClose(self.w) == 0) {
            c.glfwPollEvents();
            self.render();
        }
    }

    pub fn render(self: *Engine) void {
        var st: c.WGPUSurfaceTexture = std.mem.zeroes(c.WGPUSurfaceTexture);
        c.wgpuSurfaceGetCurrentTexture(self.surface, &st);

        if (
            st.status != c.WGPUSurfaceGetCurrentTextureStatus_SuccessOptimal and 
            st.status != c.WGPUSurfaceGetCurrentTextureStatus_SuccessSuboptimal
        ) {
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
            .clearValue = .{ .r = 0.1, .g = 0.2, .b = 0.4, .a = 1.0 }
        };

        const pass_desc: c.WGPURenderPassDescriptor = .{
            .nextInChain = null,
            .label = .{ .data = null, .length = 0 },
            .colorAttachmentCount = 1,
            .colorAttachments = &color_attachment,
            .depthStencilAttachment = null,
            .occlusionQuerySet = null,
            .timestampWrites = null
        };

        const pass = c.wgpuCommandEncoderBeginRenderPass(encoder, &pass_desc);
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
        userdata2: ?*anyopaque
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
        userdata2: ?*anyopaque
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
        userdata2: ?*anyopaque
    ) callconv(.c) void {
        _ = device;
        _ = userdata1;
        _ = userdata2;

        const text = if (message.data != null and message.length > 0)
            message.data[0..message.length] 
            else "(no message)";

        std.debug.print("wgpu error (type={d}): {s}\n", .{ err_type, text });
    }
};

test "compile-check Engine" {
    var e = Engine.init() orelse return error.InitFailed;
    defer e.deinit();

    e.run();
}
