pub const Mat4 = extern struct {
    data: [16]f32,

    pub fn identity() Mat4 {
        return .{
            .data = .{
                1, 0, 0, 0, // Column 1
                0, 1, 0, 0, // Column 2
                0, 0, 1, 0, // Column 3
                0, 0, 0, 1, // Column 4
            },
        };
    }

    /// Builds a matrix that maps a 3D box of the game world into the GPU's clip-space cube.
    /// No perspective, just scale + shift.
    ///
    /// The returned matrix is 4x4. Each row defines how one output coordinate
    /// (x, y, z, or w) is computed from the input vertex (x, y, z, 1).
    /// During rendering, this matrix is multiplied by each vertex to produce
    /// its position in clip space.
    ///
    /// **First row — computes new X**
    ///
    /// Row contents: `[2/rl, 0, 0, -(right+left)/rl]` where `rl = right - left`.
    ///
    /// Per the matrix multiplication rule:
    ///   new_x = M[0][0]*x + M[0][1]*y + M[0][2]*z + M[0][3]*1
    ///
    /// The y and z coefficients are zero, so only x and the constant contribute:
    ///   new_x = (2/rl)*x  +  (-(right+left)/rl)
    ///
    /// The first term scales the input range [left, right] to a width of 2.
    /// The second term shifts that range so it's centered on 0, landing in [-1, +1] —
    /// the X range the GPU expects in clip space.
    ///
    /// **Second row — computes new Y**
    ///
    /// Same idea as row 0, but with `bottom`/`top` instead of `left`/`right`,
    /// and the non-zero scale lives at M[1][1] instead of M[0][0].
    /// Result range: [-1, +1].
    ///
    /// **Third row — computes new Z**
    ///
    /// Same idea again, but the target range is [0, 1] instead of [-1, +1]
    /// (Vulkan/D3D clip-space Z convention). That's why the scale is `1/fn_`
    /// rather than `2/fn_`, and the shift is `-near/fn_` rather than centered.
    ///
    /// **Fourth row — computes new W**
    ///
    /// Row contents: `[0, 0, 0, 1]`, so new_w is always 1 regardless of input.
    ///
    /// This is the signature of an orthographic projection. After this matrix,
    /// the GPU performs a "perspective divide" that divides x, y, z by w.
    /// Since w is always 1, that divide changes nothing — no foreshortening,
    /// no depth-based scaling. A perspective projection would put `-1` at M[3][2]
    /// instead, making w depend on z and producing the shrinking-with-distance effect.
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) Mat4 {
        const rl = right - left;
        const tb = top - bottom;
        const fn_ = far - near;

        var m = Mat4{ .data = .{0} ** 16 };
        m.data[0] = 2.0 / rl;
        m.data[5] = 2.0 / tb;
        m.data[10] = 1.0 / fn_;
        m.data[12] = -(right + left) / rl;
        m.data[13] = -(top + bottom) / tb;
        m.data[14] = -near / fn_;
        m.data[15] = 1.0;
        return m;
    }

    pub fn translate(x: f32, y: f32, z: f32) Mat4 {
        var m = Mat4.identity();
        m.data[12] = x; // 4x1 slot
        m.data[13] = y; // 4x2 slot
        m.data[14] = z; // 4x3 slot
        return m;
    }

    pub fn mult(a: Mat4, b: Mat4) Mat4 {
        var r = Mat4{ .data = .{0} ** 16 };

        var col: usize = 0;
        while (col < 4) : (col += 1) {
            var row: usize = 0;

            while (row < 4) : (row += 1) {
                var sum: f32 = 0;
                var k: usize = 0;

                while (k < 4) : (k += 1) {
                    sum += a.data[k * 4 + row] * b.data[col * 4 + k];
                }

                r.data[col * 4 + row] = sum;
            }
        }

        return r;
    }
};
