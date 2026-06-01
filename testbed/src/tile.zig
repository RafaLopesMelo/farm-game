const std = @import("std");

pub const TileKind = enum(u8) { grass, water };

pub const Tilemap = struct {
    tiles: []TileKind,
    width: u32,
    height: u32,
    tile_size: f32,

    pub fn init(allocator: std.mem.Allocator, width: u32, height: u32, tile_size: f32) !Tilemap {
        const size: usize = @as(usize, width) * @as(usize, height);
        const tiles = try allocator.alloc(TileKind, size);
        @memset(tiles, TileKind.grass);

        return .{
            .tiles = tiles,
            .width = width,
            .height = height,
            .tile_size = tile_size,
        };
    }

    pub fn deinit(self: *Tilemap, allocator: std.mem.Allocator) void {
        allocator.free(self.tiles);
    }
};
