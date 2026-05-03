const std = @import("std");
const engine = @import("engine");

pub fn main() !void {
    var e = engine.Engine.init().?;
    defer e.deinit();
    e.run();
}
