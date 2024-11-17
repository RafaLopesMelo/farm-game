# Game Definitions

- The map is composed of chunks of size 32x32 blocks
- Each chunk is composed of blocks of size 16x16 pixels

# sRGB

Is the international standard to represent colors in software and hardware

# Primitive Topology

How to interpret the vertices. Here each set of 3 vertices composes a new triangle

# Cull Mode

Determines whether certain faces of 3D objects should be rendered or discarded.
It is used to improve performance by avoiding drawing of faces that aren't visible to the viewer

Triangles drawn in clockwise order is considered to be front-facing. So is not discarded by the culling


# Render Pass

List of render commands in a command encoder. A render pass may contain any number of drawing commands, and before/between each command the render state may be updated however you wish.

You'd usually want to bundle as much rendering as possible into a single pass per render. But if you have some rendering that depends on the output of a previous pass, the second pass is unavoidable.

# World Generation

For world generation is commonly used algorithms like `Perlin Noise`, which is a mathematical function that can be used to generate a random pattern that looks like natural.

The Perlin Noise algorithms generates a noise which is a number between -1 and 1 or 0 and 1. The noise is then mapped to a correspondent tile. 

The Perlin Noise is deterministic, which means that the same input will always generate the same output. Perlin Noise is also able to have a seed implementation.

# Textures

To load textures on the GPU is commonly used an `Atlas Texture`, which is a texture that contains multiple smaller textures, each one of them is called a `Sub Texture`.

> Maybe should I dynamically the atlas based on the tiles present in the world at the moment?

## References
https://www.reddit.com/r/rust_gamedev/comments/176ur9k/is_it_goodpractice_to_create_multiple_render/
