const c = @import("c");

pub const Input = struct {
    keys_down: [c.GLFW_KEY_LAST + 1]bool,

    pub fn init() Input {
        return .{ .keys_down = .{false} ** (c.GLFW_KEY_LAST + 1) };
    }

    pub fn isKeyDown(self: *const Input, key: c_int) bool {
        return self.keys_down[@intCast(key)];
    }

    pub fn handleKey(self: *Input, key: c_int, action: c_int) void {
        if (key < 0 or key > c.GLFW_KEY_LAST) return;

        if (action == c.GLFW_PRESS) {
            self.keys_down[@intCast(key)] = true;
            return;
        }

        if (action == c.GLFW_RELEASE) {
            self.keys_down[@intCast(key)] = false;
            return;
        }
    }
};
