const std = @import("std");

const game = @import("game.zig");

pub fn main() !void {
    var g = try game.Game.init();
    defer g.deinit();

    g.run();
}
