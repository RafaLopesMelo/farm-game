use crate::world::coords::Coords2D;

use super::perlin::PerlinNoise;

pub struct FractalNoise {
    noise: PerlinNoise,
}

impl FractalNoise {
    pub fn new() -> Self {
        return Self {
            noise: PerlinNoise::new(),
        };
    }

    pub fn generate(
        &self,
        coords: Coords2D,
        octaves: u8,
        persistence: f32,
        lacunarity: f32,
        frequency: f32,
    ) -> f32 {
        let mut noise = 0.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;
        let mut tmp_frequency = frequency;

        for _ in 0..octaves {
            noise += self.noise.generate(coords, tmp_frequency) * amplitude;
            max_value += amplitude;

            amplitude *= persistence; // Decrease amplitude
            tmp_frequency *= lacunarity; // Increase frequency
        }

        return noise / max_value;
    }
}
