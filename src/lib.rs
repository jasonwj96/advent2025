#![cfg_attr(feature = "simd", allow(unstable_features), feature(portable_simd))]

macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {$(pub mod $day;)*}
    }
}

library!(util "Utility modules to handle common recurring Advent of Code patterns."
    ansi, bitset, grid, hash, heap, integer, iter, math, md5, parse, point, slice, thread
);

library!(year2025 "Advent 2025 solutions"
    day01
);