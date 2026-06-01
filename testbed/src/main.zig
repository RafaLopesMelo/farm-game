const std = @import("std");
const engine = @import("engine");
const c = @import("engine").c;
const tile = @import("tile.zig");

var player_x: f32 = 100;
var player_y: f32 = 100;
const speed: f64 = 1024;

fn uvForKind(kind: tile.TileKind) engine.UvRect {
    return switch (kind) {
        .grass => .{ .u0 = 0, .v0 = 0, .u1 = 0.5, .v1 = 1 },
        .water => .{ .u0 = 0.5, .v0 = 0, .u1 = 1, .v1 = 1 },
    };
}

pub fn main() !void {
    const tilemap = try tile.Tilemap.init(std.heap.page_allocator, 10, 10, 64);

    var e = engine.Engine.init().?;
    defer e.deinit();
    e.run();

    while (e.shouldClose()) {
        e.pollEvents();
        e.beginFrame();

        const delta = e.getDelta();

        if (e.input.isKeyDown(engine.Key.w)) player_y -= @floatCast(speed * delta);
        if (e.input.isKeyDown(engine.Key.s)) player_y += @floatCast(speed * delta);
        if (e.input.isKeyDown(engine.Key.a)) player_x -= @floatCast(speed * delta);
        if (e.input.isKeyDown(engine.Key.d)) player_x += @floatCast(speed * delta);

        e.setCamera(player_x, player_y);

        for (tilemap.tiles, 0..) |kind, i| {
            const x: f32 = @floatFromInt(i % tilemap.width);
            const y: f32 = @floatFromInt(i / tilemap.height);

            e.drawSprite(
                x * tilemap.tile_size,
                y * tilemap.tile_size,
                tilemap.tile_size,
                tilemap.tile_size,
                uvForKind(kind),
            );
        }

        e.drawSprite(player_x, player_y, 64, 64, .{ .u0 = 0.5, .v0 = 0, .u1 = 0.5, .v1 = 1 });

        e.endFrame();
    }
}
