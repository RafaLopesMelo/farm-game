use super::{Texture, TextureAtlas};
use crate::world::tiles::{grass::GrassTile, Tile};

pub fn texture_for_tile(t: &dyn Tile, neighbors: Option<[&dyn Tile; 4]>) -> Texture {
    let tile = t
        .as_any()
        .downcast_ref::<GrassTile>()
        .expect("cannot cast tile to GrassTile");

    if neighbors.is_none() {
        return TextureAtlas::texture_from_coords([0, 0]);
    }

    let coords = tile.coords();

    let n = neighbors.unwrap();

    let top = n[0];
    let right = n[1];
    let bottom = n[2];
    let left = n[3];

    let as_high_r = coords.as_high_as(right.coords());
    let as_high_l = coords.as_high_as(left.coords());
    let as_high_t = coords.as_high_as(top.coords());
    let as_high_b = coords.as_high_as(bottom.coords());

    let higher_than_r = coords.higher_than(right.coords());
    let higher_than_l = coords.higher_than(left.coords());
    let higher_than_t = coords.higher_than(top.coords());
    let higher_than_b = coords.higher_than(bottom.coords());

    if higher_than_b {
        if as_high_r && as_high_l {
            return TextureAtlas::texture_from_coords([0, 1]);
        }

        if as_high_l {
            println!("as high l, {} {}", coords.z(), bottom.coords().z());
            return TextureAtlas::texture_from_coords([1, 0]);
        }

        if as_high_r {
            return TextureAtlas::texture_from_coords([8, 0]);
        }

        return TextureAtlas::texture_from_coords([1, 1]);
    }

    if higher_than_t {
        if as_high_r && as_high_l {
            return TextureAtlas::texture_from_coords([3, 0]);
        }

        if as_high_l {
            return TextureAtlas::texture_from_coords([5, 0]);
        }

        if as_high_r {
            return TextureAtlas::texture_from_coords([2, 0]);
        }

        return TextureAtlas::texture_from_coords([1, 1]);
    }

    if higher_than_r && higher_than_l {
        if as_high_t && as_high_b {
            return TextureAtlas::texture_from_coords([2, 1]);
        }

        if as_high_t {
            return TextureAtlas::texture_from_coords([3, 1]);
        }

        if as_high_b {
            return TextureAtlas::texture_from_coords([1, 1]);
        }

        panic!("invalid tile condition");
    }

    if higher_than_r {
        if as_high_t && as_high_b {
            return TextureAtlas::texture_from_coords([7, 0]);
        }
    }

    if higher_than_l {
        if as_high_t && as_high_b {
            return TextureAtlas::texture_from_coords([6, 0]);
        }
    }

    return TextureAtlas::texture_from_coords([0, 0]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{
        coords::Coords3D,
        tiles::{dirt::DirtTile, water::WaterTile},
    };

    #[test]
    fn test_default_grass() {
        let tile = GrassTile::new(Coords3D::new_lattice(0, 0, 1));
        let texture = texture_for_tile(&tile, None);
        let expected = TextureAtlas::texture_from_coords([0, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_plain_grass() {
        let tile = GrassTile::new(Coords3D::new_lattice(0, 0, 0));
        let neighbors: [&dyn Tile; 4] = [
            &DirtTile::new(Coords3D::new_lattice(0, 0, 0)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 0)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 0)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 0)),
        ];

        let texture = texture_for_tile(&tile, Some(neighbors));
        let expected = TextureAtlas::texture_from_coords([0, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_invalid_grass() {
        let r = std::panic::catch_unwind(|| {
            let tile = WaterTile::new(Coords3D::new_lattice(0, 0, 0));
            texture_for_tile(&tile, None);
        });

        assert!(r.is_err());
    }

    #[test]
    fn test_lower_bottom_others_same_height() {
        let tile = GrassTile::new(Coords3D::new_lattice(0, 0, 1));
        let neighbors: [&dyn Tile; 4] = [
            &DirtTile::new(Coords3D::new_lattice(0, 0, 1)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 1)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 0)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 1)),
        ];

        let texture = texture_for_tile(&tile, Some(neighbors));
        let expected = TextureAtlas::texture_from_coords([0, 1]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_left_others_same_height() {
        let tile = GrassTile::new(Coords3D::new_lattice(0, 0, 1));
        let neighbors: [&dyn Tile; 4] = [
            &DirtTile::new(Coords3D::new_lattice(0, 0, 1)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 1)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 1)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 0)),
        ];

        let texture = texture_for_tile(&tile, Some(neighbors));
        let expected = TextureAtlas::texture_from_coords([6, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_right_others_same_height() {
        let tile = GrassTile::new(Coords3D::new_lattice(0, 0, 1));
        let neighbors: [&dyn Tile; 4] = [
            &DirtTile::new(Coords3D::new_lattice(0, 0, 1)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 0)),
            &WaterTile::new(Coords3D::new_lattice(0, 0, 1)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 1)),
        ];

        let texture = texture_for_tile(&tile, Some(neighbors));
        let expected = TextureAtlas::texture_from_coords([7, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_right_and_left_others_same_height() {
        let tile = GrassTile::new(Coords3D::new_lattice(0, 0, 1));
        let neighbors: [&dyn Tile; 4] = [
            &DirtTile::new(Coords3D::new_lattice(0, 0, 1)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 0)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 1)),
            &DirtTile::new(Coords3D::new_lattice(0, 0, 0)),
        ];

        let texture = texture_for_tile(&tile, Some(neighbors));
        let expected = TextureAtlas::texture_from_coords([2, 1]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_right_and_bottom_others_same_height() {
        let tile = GrassTile::new(Coords3D::new_lattice(0, 0, 1));
        let neighbors: [&dyn Tile; 4] = [
            &GrassTile::new(Coords3D::new_lattice(0, 0, 1)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 0)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 0)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 1)),
        ];

        let texture = texture_for_tile(&tile, Some(neighbors));
        let expected = TextureAtlas::texture_from_coords([1, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_right_and_top_others_same_height() {
        let tile = GrassTile::new(Coords3D::new_lattice(0, 0, 1));
        let neighbors: [&dyn Tile; 4] = [
            &GrassTile::new(Coords3D::new_lattice(0, 0, 0)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 0)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 1)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 1)),
        ];

        let texture = texture_for_tile(&tile, Some(neighbors));
        let expected = TextureAtlas::texture_from_coords([5, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_left_and_top_others_same_height() {
        let tile = GrassTile::new(Coords3D::new_lattice(0, 0, 1));
        let neighbors: [&dyn Tile; 4] = [
            &GrassTile::new(Coords3D::new_lattice(0, 0, 0)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 1)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 1)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 0)),
        ];

        let texture = texture_for_tile(&tile, Some(neighbors));
        let expected = TextureAtlas::texture_from_coords([2, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_left_and_bottom_others_same_height() {
        let tile = GrassTile::new(Coords3D::new_lattice(0, 0, 1));
        let neighbors: [&dyn Tile; 4] = [
            &GrassTile::new(Coords3D::new_lattice(0, 0, 1)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 1)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 0)),
            &GrassTile::new(Coords3D::new_lattice(0, 0, 0)),
        ];

        let texture = texture_for_tile(&tile, Some(neighbors));
        let expected = TextureAtlas::texture_from_coords([8, 0]);

        assert_eq!(texture, expected);
    }
}
