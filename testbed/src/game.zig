const std = @import("std");

const engine = @import("engine");
const tile = @import("tile.zig");

fn uvForKind(kind: tile.TileKind) engine.UvRect {
    return switch (kind) {
        .grass => .{ .u0 = 0, .v0 = 0, .u1 = 0.5, .v1 = 1 },
        .water => .{ .u0 = 0.5, .v0 = 0, .u1 = 1, .v1 = 1 },
    };
}

pub const Game = struct {
    engine: engine.Engine,
    tilemap: tile.Tilemap,

    player_x: f32,
    player_y: f32,
    speed: f32,

    pub fn init() !Game {
        const e = engine.Engine.init().?;
        const tilemap = try tile.Tilemap.init(std.heap.page_allocator, 10, 10, 64);

        return .{
            .engine = e,
            .tilemap = tilemap,

            .player_x = 100,
            .player_y = 100,
            .speed = 100,
        };
    }

    pub fn deinit(self: *Game) void {
        self.engine.deinit();
        self.tilemap.deinit(std.heap.page_allocator);
    }

    pub fn run(self: *Game) void {
        self.engine.run();

        while (!self.engine.shouldClose()) {
            self.engine.pollEvents();

            self.engine.beginFrame();

            const delta = self.engine.getDelta();

            if (self.engine.input.isKeyDown(engine.Key.w)) {
                self.player_y -= @floatCast(self.speed * delta);
            }

            if (self.engine.input.isKeyDown(engine.Key.s)) {
                self.player_y += @floatCast(self.speed * delta);
            }

            if (self.engine.input.isKeyDown(engine.Key.a)) {
                self.player_x -= @floatCast(self.speed * delta);
            }

            if (self.engine.input.isKeyDown(engine.Key.d)) {
                self.player_x += @floatCast(self.speed * delta);
            }

            self.engine.setCamera(self.player_x, self.player_y);

            for (self.tilemap.tiles, 0..) |kind, i| {
                const x: f32 = @floatFromInt(i % self.tilemap.width);
                const y: f32 = @floatFromInt(i / self.tilemap.width);

                self.engine.drawSprite(
                    x * self.tilemap.tile_size,
                    y * self.tilemap.tile_size,
                    self.tilemap.tile_size,
                    self.tilemap.tile_size,
                    uvForKind(kind),
                );
            }

            self.engine.drawSprite(
                self.player_x,
                self.player_y,
                64,
                64,
                .{ .u0 = 0.5, .v0 = 0, .u1 = 1, .v1 = 1 },
            );

            self.engine.endFrame();
        }
    }
};
