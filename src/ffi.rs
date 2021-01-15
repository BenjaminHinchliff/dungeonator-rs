#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    // just tests that nothing crashes
    #[test]
    fn sanity_test() {
        unsafe {
            seedDungeonatorRNG();

            let mut grid = mem::zeroed();
            let success = generateDungeon(&mut grid as *mut _, 51, 51, 1000, 2);
            assert!(success);
            freeGrid(&mut grid as *mut _);
        }
    }
}
