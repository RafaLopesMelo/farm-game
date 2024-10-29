# Game Definitions

- The map is composed of chunks of size 32x32 blocks
- Each chunk is composed of blocks of size 16x16 pixels

# sRGB

Is the international standard to represent colors in software and hardware

# Render Pass

List of render commands in a command encoder. A render pass may contain any number of drawing commands, and before/between each command the render state may be updated however you wish.

You'd usually want to bundle as much rendering as possible into a single pass per render. But if you have some rendering that depends on the output of a previous pass, the second pass is unavoidable.

## References
https://www.reddit.com/r/rust_gamedev/comments/176ur9k/is_it_goodpractice_to_create_multiple_render/
