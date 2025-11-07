# Game Requirements

Tiles are every piece of world in the game
Every tile will have a type, like Grass, Dirt, Water, and so on
Every tile type can have its own property, Water can have temperature for example
Every tile may have animatios or not
The tiles are be grouped in chunks, every 64x64 tiles group is a chunk
Some entities of the game can fill multiple tiles. A big furnace may fit 4x4 tiles group
The tiles needs to have elevation, so some tiles are above others
The player can interact with every tile, dirt can be plewed for planting, for example
Some tiles may be walked over by the player, others dont
Each tile has its own coordinate, in x y z format 
The world size of each tile is slightly bigger than the player, so fractional coordinate is required

# Implementation Thoughts

All tiles textures are placed in a large atlas, each tile have its own UV coordinate
