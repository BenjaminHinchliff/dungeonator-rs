use enum_primitive::{
    enum_from_primitive, enum_from_primitive_impl, enum_from_primitive_impl_ty, FromPrimitive,
};
use std::convert::TryInto;
use std::fmt;
use std::mem;
use std::slice;
use std::sync::Once;

mod ffi;

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(i32)]
    pub enum Tile {
        Wall = ffi::Tiles_WALL,
        Floor = ffi::Tiles_FLOOR,
        Door = ffi::Tiles_DOOR,
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => '#',
                Tile::Floor => '.',
                Tile::Door => '+',
            }
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to Generate Dungeon")]
    DungeonGenerationFailure,
}

pub struct Grid(Vec<Vec<Tile>>);

impl From<ffi::Grid> for Grid {
    fn from(mut grid: ffi::Grid) -> Self {
        Self(unsafe {
            let converted = slice::from_raw_parts(
                grid.data,
                grid.height.try_into().expect("grid height was negative!"),
            )
            .iter()
            .map(|&s| {
                slice::from_raw_parts::<ffi::Tile>(
                    s,
                    grid.width.try_into().expect("grid width was negative!"),
                )
                .iter()
                .map(|&e| Tile::from_i32(e).expect("invalid tile in grid!"))
                .collect::<Vec<_>>()
            })
            .collect();
            ffi::freeGrid(&mut grid as *mut _);
            converted
        })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

static RNG_SEEDED: Once = Once::new();

pub fn generate_dungeon(
    width: i32,
    height: i32,
    place_tries: i32,
    additional_size: i32,
) -> Result<Grid, Error> {
    RNG_SEEDED.call_once(|| unsafe {
        ffi::seedDungeonatorRNG();
    });
    let (success, grid) = unsafe {
        let mut grid: ffi::Grid = mem::zeroed();
        let success = ffi::generateDungeon(
            &mut grid as *mut _,
            width,
            height,
            place_tries,
            additional_size,
        );
        (success, grid)
    };
    if success {
        Ok(grid.into())
    } else {
        Err(Error::DungeonGenerationFailure)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn ffi_grid_to_safe_grid_works() {
        let ffi_grid: ffi::Grid = unsafe {
            let mut ffi_grid = mem::zeroed();
            let success = ffi::createGrid(5, 3, &mut ffi_grid as *mut _);
            assert!(success);
            ffi_grid
        };
        let safe_grid: Grid = ffi_grid.into();
        safe_grid
            .0
            .iter()
            .flatten()
            .for_each(|&e| assert!(e == Tile::Wall));
    }

    #[test]
    fn generate_dungeon_works() {
        generate_dungeon(51, 23, 1000, 2).unwrap();
    }
}
