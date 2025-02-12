use super::{Texture, TextureAtlas};
use crate::world::{
    coords::Coords2D,
    tiles::{grass::GrassTile, Tile, TileKind},
    world::TileLocator,
};

pub fn texture_for_tile(t: &dyn Tile, locator: &dyn TileLocator) -> Option<Texture> {
    let tile = t
        .as_any()
        .downcast_ref::<GrassTile>()
        .expect("cannot cast tile to GrassTile");

    let coords = tile.coords();

    let top = analyze_top(
        tile,
        locator.tile_at(Coords2D::new(coords.x(), coords.y() + 1.0))?,
    );

    let right = analyze_right(
        tile,
        locator.tile_at(Coords2D::new(coords.x() + 1.0, coords.y()))?,
    );

    let bottom = analyze_bottom(
        tile,
        locator.tile_at(Coords2D::new(coords.x(), coords.y() - 1.0))?,
    );

    let left = analyze_left(
        tile,
        locator.tile_at(Coords2D::new(coords.x() - 1.0, coords.y()))?,
    );

    let intersection = top
        .iter()
        .filter(|&texture| {
            right.contains(texture) && bottom.contains(texture) && left.contains(texture)
        })
        .next()
        .cloned()
        .or(Some(TextureAtlas::texture_from_coords([0, 0])))
        .expect("no textures intersection");

    return Some(intersection);
}

fn analyze_left(c: &GrassTile, l: &dyn Tile) -> Vec<Texture> {
    let mut choices: Vec<Texture> = vec![];

    if l.is(TileKind::Water) {
        choices.push(TextureAtlas::texture_from_coords([9, 4]));
    } else {
        let coords = c.coords();

        if coords.higher_than(l.coords()) {
            choices.push(TextureAtlas::texture_from_coords([2, 0]));
            choices.push(TextureAtlas::texture_from_coords([1, 1]));
            choices.push(TextureAtlas::texture_from_coords([2, 1]));
            choices.push(TextureAtlas::texture_from_coords([3, 1]));
            choices.push(TextureAtlas::texture_from_coords([6, 0]));
            choices.push(TextureAtlas::texture_from_coords([8, 0]));
        }

        if coords.as_high_as(l.coords()) {
            choices.push(TextureAtlas::texture_from_coords([0, 0]));
            choices.push(TextureAtlas::texture_from_coords([5, 0]));
            choices.push(TextureAtlas::texture_from_coords([2, 5]));
            choices.push(TextureAtlas::texture_from_coords([7, 0]));
            choices.push(TextureAtlas::texture_from_coords([1, 0]));
            choices.push(TextureAtlas::texture_from_coords([9, 0]));
            choices.push(TextureAtlas::texture_from_coords([3, 0]));
            choices.push(TextureAtlas::texture_from_coords([0, 1]));
            choices.push(TextureAtlas::texture_from_coords([0, 4]));
            choices.push(TextureAtlas::texture_from_coords([1, 3]));
        }
    }

    return choices;
}

fn analyze_right(c: &GrassTile, r: &dyn Tile) -> Vec<Texture> {
    let mut choices: Vec<Texture> = vec![];

    if r.is(TileKind::Water) {
        choices.push(TextureAtlas::texture_from_coords([0, 5]));
    } else {
        let coords = c.coords();

        if coords.higher_than(r.coords()) {
            choices.push(TextureAtlas::texture_from_coords([5, 0]));
            choices.push(TextureAtlas::texture_from_coords([7, 0]));
            choices.push(TextureAtlas::texture_from_coords([1, 1]));
            choices.push(TextureAtlas::texture_from_coords([2, 1]));
            choices.push(TextureAtlas::texture_from_coords([3, 1]));
            choices.push(TextureAtlas::texture_from_coords([1, 0]));
        }

        if coords.as_high_as(r.coords()) {
            choices.push(TextureAtlas::texture_from_coords([0, 0]));
            choices.push(TextureAtlas::texture_from_coords([2, 5]));
            choices.push(TextureAtlas::texture_from_coords([2, 0]));
            choices.push(TextureAtlas::texture_from_coords([3, 0]));
            choices.push(TextureAtlas::texture_from_coords([6, 0]));
            choices.push(TextureAtlas::texture_from_coords([8, 0]));
            choices.push(TextureAtlas::texture_from_coords([0, 1]));
            choices.push(TextureAtlas::texture_from_coords([0, 4]));
        }
    }

    return choices;
}

fn analyze_bottom(c: &GrassTile, b: &dyn Tile) -> Vec<Texture> {
    let mut choices: Vec<Texture> = vec![];

    if b.is(TileKind::Water) {
        choices.push(TextureAtlas::texture_from_coords([0, 4]));
    } else {
        let coords = c.coords();

        if coords.higher_than(b.coords()) {
            choices.push(TextureAtlas::texture_from_coords([0, 1]));
            choices.push(TextureAtlas::texture_from_coords([3, 1]));
            choices.push(TextureAtlas::texture_from_coords([8, 0]));
            choices.push(TextureAtlas::texture_from_coords([1, 0]));
            choices.push(TextureAtlas::texture_from_coords([9, 0]));
        }

        if coords.as_high_as(b.coords()) {
            choices.push(TextureAtlas::texture_from_coords([0, 0]));
            choices.push(TextureAtlas::texture_from_coords([2, 0]));
            choices.push(TextureAtlas::texture_from_coords([2, 5]));
            choices.push(TextureAtlas::texture_from_coords([3, 0]));
            choices.push(TextureAtlas::texture_from_coords([5, 0]));
            choices.push(TextureAtlas::texture_from_coords([6, 0]));
            choices.push(TextureAtlas::texture_from_coords([7, 0]));
            choices.push(TextureAtlas::texture_from_coords([1, 1]));
            choices.push(TextureAtlas::texture_from_coords([2, 1]));
        }
    }

    return choices;
}

fn analyze_top(c: &GrassTile, t: &dyn Tile) -> Vec<Texture> {
    let mut choices: Vec<Texture> = vec![];

    if t.is(TileKind::Water) {
        choices.push(TextureAtlas::texture_from_coords([2, 5]));
    } else {
        let coords = c.coords();

        if coords.higher_than(t.coords()) {
            choices.push(TextureAtlas::texture_from_coords([2, 0]));
            choices.push(TextureAtlas::texture_from_coords([3, 0]));
            choices.push(TextureAtlas::texture_from_coords([5, 0]));
            choices.push(TextureAtlas::texture_from_coords([1, 1]));
        }

        if coords.as_high_as(t.coords()) {
            choices.push(TextureAtlas::texture_from_coords([0, 0]));
            choices.push(TextureAtlas::texture_from_coords([2, 1]));
            choices.push(TextureAtlas::texture_from_coords([6, 0]));
            choices.push(TextureAtlas::texture_from_coords([0, 1]));
            choices.push(TextureAtlas::texture_from_coords([8, 0]));
            choices.push(TextureAtlas::texture_from_coords([1, 0]));
            choices.push(TextureAtlas::texture_from_coords([7, 0]));
            choices.push(TextureAtlas::texture_from_coords([0, 4]));
        }
    }

    return choices;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::world::{
        coords::Coords3D,
        tiles::{dirt::DirtTile, water::WaterTile},
    };

    fn get_base_neighbors(center: &Coords3D, height: i32) -> Vec<Box<dyn Tile>> {
        vec![
            Box::from(GrassTile::new(Coords3D::new_lattice(
                center.lattice_x(),
                center.lattice_y() + 1,
                height,
            ))), // top
            Box::from(GrassTile::new(Coords3D::new_lattice(
                center.lattice_x() + 1,
                center.lattice_y(),
                height,
            ))), // right
            Box::from(GrassTile::new(Coords3D::new_lattice(
                center.lattice_x(),
                center.lattice_y() - 1,
                height,
            ))), // bottom
            Box::from(GrassTile::new(Coords3D::new_lattice(
                center.lattice_x() - 1,
                center.lattice_y(),
                height,
            ))), // left
        ]
    }

    struct FakeTileLocator {
        tiles: HashMap<Coords2D, Box<dyn Tile>>,
    }

    impl FakeTileLocator {
        pub fn new(tiles: Vec<Box<dyn Tile>>) -> Self {
            let mut h = HashMap::new();

            for tile in tiles {
                let coords = tile.coords();
                h.insert(coords.to_2d(), tile);
            }

            return Self { tiles: h };
        }
    }

    impl TileLocator for FakeTileLocator {
        fn tile_at(&self, coords: Coords2D) -> Option<&dyn Tile> {
            let el = self.tiles.get(&coords)?;
            return Some(el.as_ref());
        }
    }

    #[test]
    fn test_default_grass() {
        let tile = GrassTile::new(Coords3D::new_lattice(0, 0, 1));
        let texture = texture_for_tile(&tile, &FakeTileLocator::new(vec![]));

        assert_eq!(texture, None);
    }

    #[test]
    fn test_plain_grass() {
        let center = Coords3D::new_lattice(0, 0, 0);
        let tile = GrassTile::new(center);
        let neighbors = get_base_neighbors(&center, 0);

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([0, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_invalid_grass() {
        let r = std::panic::catch_unwind(|| {
            let tile = WaterTile::new(Coords3D::new_lattice(0, 0, 0));
            texture_for_tile(&tile, &FakeTileLocator::new(vec![]));
        });

        assert!(r.is_err());
    }

    #[test]
    fn test_lower_bottom_others_same_height() {
        let center = Coords3D::new_lattice(0, 0, 1);
        let tile = GrassTile::new(center);
        let mut neighbors = get_base_neighbors(&center, 1);
        neighbors[2] = Box::from(DirtTile::new(Coords3D::new_lattice(0, -1, 0))); // Set bottom neighbor lower

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([0, 1]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_top_others_same_height() {
        let center = Coords3D::new_lattice(0, 0, 1);
        let tile = GrassTile::new(center);
        let mut neighbors = get_base_neighbors(&center, 1);
        neighbors[0] = Box::from(DirtTile::new(Coords3D::new_lattice(0, 1, 0))); // Set top neighbor lower

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([3, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_left_others_same_height() {
        let center = Coords3D::new_lattice(0, 0, 1);
        let tile = GrassTile::new(center);
        let mut neighbors = get_base_neighbors(&center, 1);
        neighbors[3] = Box::from(DirtTile::new(Coords3D::new_lattice(-1, 0, 0))); // Set left neighbor lower

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([6, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_right_others_same_height() {
        let center = Coords3D::new_lattice(0, 0, 1);
        let tile = GrassTile::new(center);
        let mut neighbors = get_base_neighbors(&center, 1);
        neighbors[1] = Box::from(DirtTile::new(Coords3D::new_lattice(1, 0, 0))); // Set right neighbor lower

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([7, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_right_and_left_others_same_height() {
        let center = Coords3D::new_lattice(0, 0, 1);
        let tile = GrassTile::new(center);
        let mut neighbors = get_base_neighbors(&center, 1);
        neighbors[1] = Box::from(DirtTile::new(Coords3D::new_lattice(1, 0, 0))); // Set right neighbor lower
        neighbors[3] = Box::from(DirtTile::new(Coords3D::new_lattice(-1, 0, 0))); // Set left neighbor lower

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([2, 1]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_right_and_bottom_others_same_height() {
        let center = Coords3D::new_lattice(0, 0, 1);
        let tile = GrassTile::new(center);
        let mut neighbors = get_base_neighbors(&center, 1);
        neighbors[1] = Box::from(GrassTile::new(Coords3D::new_lattice(1, 0, 0))); // Set right neighbor lower
        neighbors[2] = Box::from(GrassTile::new(Coords3D::new_lattice(0, -1, 0))); // Set bottom neighbor lower

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([1, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_right_and_top_others_same_height() {
        let center = Coords3D::new_lattice(0, 0, 1);
        let tile = GrassTile::new(center);
        let mut neighbors = get_base_neighbors(&center, 1);
        neighbors[0] = Box::from(GrassTile::new(Coords3D::new_lattice(0, 1, 0))); // Set top neighbor lower
        neighbors[1] = Box::from(GrassTile::new(Coords3D::new_lattice(1, 0, 0))); // Set right neighbor lower

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([5, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_left_and_top_others_same_height() {
        let center = Coords3D::new_lattice(0, 0, 1);
        let tile = GrassTile::new(center);
        let mut neighbors = get_base_neighbors(&center, 1);
        neighbors[0] = Box::from(GrassTile::new(Coords3D::new_lattice(0, 1, 0))); // Set top neighbor lower
        neighbors[3] = Box::from(GrassTile::new(Coords3D::new_lattice(-1, 0, 0))); // Set left neighbor lower

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([2, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_lower_left_and_bottom_others_same_height() {
        let center = Coords3D::new_lattice(0, 0, 1);
        let tile = GrassTile::new(center);
        let mut neighbors = get_base_neighbors(&center, 1);
        neighbors[2] = Box::from(GrassTile::new(Coords3D::new_lattice(0, -1, 0))); // Set bottom neighbor lower
        neighbors[3] = Box::from(GrassTile::new(Coords3D::new_lattice(-1, 0, 0))); // Set left neighbor lower

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([8, 0]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_bottom_water_same_height() {
        let center = Coords3D::new_lattice(0, 0, 0);
        let tile = GrassTile::new(center);
        let mut neighbors = get_base_neighbors(&center, 0);
        neighbors[2] = Box::from(WaterTile::new(Coords3D::new_lattice(0, -1, 0))); // Set bottom neighbor to water

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([0, 4]);

        assert_eq!(texture, expected);
    }

    #[test]
    fn test_top_water_same_height() {
        let center = Coords3D::new_lattice(0, 0, 0);
        let tile = GrassTile::new(center);
        let mut neighbors = get_base_neighbors(&center, 0);
        neighbors[0] = Box::from(WaterTile::new(Coords3D::new_lattice(0, 1, 0))); // Set top neighbor to water

        let texture = texture_for_tile(&tile, &FakeTileLocator::new(neighbors)).unwrap();
        let expected = TextureAtlas::texture_from_coords([2, 5]);

        assert_eq!(texture, expected);
    }
}
