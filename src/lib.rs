use std::convert::TryInto;
use std::slice;

use enum_primitive::{
    enum_from_primitive, enum_from_primitive_impl, enum_from_primitive_impl_ty, FromPrimitive,
};

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

struct Grid(Vec<Vec<Tile>>);

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
                ).iter().map(|&e| Tile::from_i32(e).expect("invalid tile in grid!")).collect::<Vec<_>>()
            }).collect();
            ffi::freeGrid(&mut grid as *mut _);
            converted
        })
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
        safe_grid.0.iter().flatten().for_each(|&e| assert!(e == Tile::Wall));
    }
}
