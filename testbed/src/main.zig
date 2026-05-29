const std = @import("std");
const engine = @import("engine");
const c = @import("engine").c;

var player_x: f32 = 100;
var player_y: f32 = 100;
const speed: f64 = 1024;

pub fn main() !void {
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

        var row: f32 = 0;
        while (row < 10) : (row += 1) {
            var col: f32 = 0;
            while (col < 5) : (col += 1) {
                const u_offset: f32 = if (@rem(col, 2.0) == 0.0) 0.0 else 0.5;
                e.drawSprite(col * 64, row * 64, 64, 64, .{
                    .u0 = u_offset,
                    .v0 = 0,
                    .u1 = u_offset + 0.5,
                    .v1 = 1,
                });
            }
        }
        e.endFrame();
    }
}
