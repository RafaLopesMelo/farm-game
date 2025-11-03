#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UvRect {
    uvwh: glam::Vec4,
}

impl UvRect {
    #[inline]
    pub fn new(u: f32, v: f32, w: f32, h: f32) -> Self {
        let uvwh = glam::Vec4::new(u, v, w, h);

        for i in 0..4 {
            let val = uvwh[i];
            if val > 1.0 || val < 0.0 {
                panic!("UvRect value out of the bounds: {}", val);
            }
        }

        return Self { uvwh };
    }

    #[inline]
    pub fn from_pixels(coords: [u32; 2], size: [u32; 2], total_size: [u32; 2]) -> Self {
        let w = (size[0] as f32) / (total_size[0] as f32);
        let h = (size[1] as f32) / (total_size[1] as f32);

        let u = (coords[0] as f32) / (total_size[0] as f32);
        let v = (coords[1] as f32) / (total_size[1] as f32);

        return UvRect::new(u, v, w, h);
    }

    pub fn to_array(&self) -> [f32; 4] {
        return self.uvwh.to_array();
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    #[test]
    fn test_new_valid() {
        let cases = vec![
            [0.0, 0.0, 0.0, 0.0],
            [0.1, 0.2, 0.3, 0.4],
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 1.0, 0.5, 0.5],
            [0.25, 0.75, 0.125, 0.875],
        ];

        for (i, v) in cases.iter().enumerate() {
            let rect = UvRect::new(v[0], v[1], v[2], v[3]);
            let arr = rect.to_array();

            for n in 0..v.len() {
                assert!(
                    v[n] == arr[n],
                    "case #{i}: got {:?}, expected {:?}",
                    arr[n],
                    v[n]
                );
            }
        }
    }

    #[test]
    fn test_panic_on_out_of_the_bounds() {
        let tt = vec![
            [2.0, 0.0, 0.5, 0.5],
            [0.0, 2.0, 0.5, 0.5],
            [0.0, 0.0, 2.0, 0.5],
            [0.0, 0.0, 0.5, 2.0],
        ];

        for t in tt {
            let result = panic::catch_unwind(|| {
                UvRect::new(t[0], t[1], t[2], t[3]);
            });

            assert!(result.is_err());
        }
    }

    #[test]
    fn test_from_pixels_valid() {
        let total = [100, 100];

        let cases = [
            ([0, 0], [0, 0]),
            ([10, 20], [30, 40]),
            ([100, 100], [100, 100]),
            ([0, 100], [50, 50]),
            ([25, 75], [12, 88]),
        ];

        let expected = vec![
            [0.0, 0.0, 0.0, 0.0],
            [0.1, 0.2, 0.3, 0.4],
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 1.0, 0.5, 0.5],
            [0.25, 0.75, 0.12, 0.88],
        ];

        for (i, v) in cases.iter().enumerate() {
            let rect = UvRect::from_pixels(v.0, v.1, total);
            let arr = rect.to_array();

            for n in 0..arr.len() {
                assert!(
                    expected[i][n] == arr[n],
                    "case #{i}: got {:?}, expected {:?}",
                    arr[n],
                    expected[i][n]
                );
            }
        }
    }
}
