use crate::world::coords::Coords2D;

pub struct PerlinNoise {}

const PERMUTATION: [usize; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

impl PerlinNoise {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn generate(&self, coords: Coords2D, frequency: f32) -> f32 {
        let x = coords.x() as f32 * frequency;
        let y = coords.y() as f32 * frequency;

        let x0 = x.floor() as i32;
        let x1 = x0 + 1;
        let y0 = y.floor() as i32;
        let y1 = y0 + 1;

        let rel_x = x - (x0 as f32);
        let rel_y = y - (y0 as f32);

        let u = Self::fade(rel_x);
        let v = Self::fade(rel_y);

        let tl = self.permute(x0, y0);
        let tr = self.permute(x1, y0);
        let bl = self.permute(x0, y1);
        let br = self.permute(x1, y1);

        let grad_tl = Self::random_gradient_vector(tl);
        let grad_tr = Self::random_gradient_vector(tr);
        let grad_bl = Self::random_gradient_vector(bl);
        let grad_br = Self::random_gradient_vector(br);

        let dot_tl = grad_tl.0 * rel_x + grad_tl.1 * rel_y;
        let dot_tr = grad_tr.0 * (rel_x - 1.0) + grad_tr.1 * rel_y;
        let dot_bl = grad_bl.0 * rel_x + grad_bl.1 * (rel_y - 1.0);
        let dot_br = grad_br.0 * (rel_x - 1.0) + grad_br.1 * (rel_y - 1.0);

        let lerp_top = Self::lerp(u, dot_tl, dot_tr);
        let lerp_bottom = Self::lerp(u, dot_bl, dot_br);

        return Self::lerp(v, lerp_top, lerp_bottom);
    }

    fn permute(&self, x: i32, y: i32) -> usize {
        let xi = (x & 255) as usize;
        let yi = (y & 255) as usize;

        return PERMUTATION[(xi + PERMUTATION[yi]) & 255];
    }

    fn random_gradient_vector(hash: usize) -> (f32, f32) {
        match hash & 7 {
            0 => (1.0, 0.0),
            1 => (-1.0, 0.0),
            2 => (0.0, 1.0),
            3 => (0.0, -1.0),
            4 => (1.0, 1.0),
            5 => (-1.0, 1.0),
            6 => (1.0, -1.0),
            7 => (-1.0, -1.0),
            _ => (0.0, 0.0),
        }
    }

    fn fade(t: f32) -> f32 {
        return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
    }

    fn lerp(t: f32, a: f32, b: f32) -> f32 {
        return a + t * (b - a);
    }
}
