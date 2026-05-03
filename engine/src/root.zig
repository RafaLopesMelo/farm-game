const std = @import("std");
pub const c = @import("c");

pub const Engine = struct {
    w: *c.GLFWwindow,

    pub fn init() ?Engine {
        if (c.glfwInit() == 0) {
            return null;
        }

        const w = c.glfwCreateWindow(720, 480, "Title", null, null) orelse return null;

        return .{ .w = w };
    }

    pub fn deinit(self: *Engine) void {
        c.glfwDestroyWindow(self.w);
        c.glfwTerminate();
    }

    pub fn run(self: *Engine) void {
        while (c.glfwWindowShouldClose(self.w) == 0) {
            c.glfwPollEvents();
        }
    }
};

test "compile-check Engine" {
    var e = Engine.init() orelse return error.InitFailed;
    defer e.deinit();

    e.run();
}
