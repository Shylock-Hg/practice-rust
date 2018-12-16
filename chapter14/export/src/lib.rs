//! # Art
//!
//! A library for modeling artistic concepts.

// export the API in new architecture
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
        /// The primary colors according to the RYB color model.
        pub enum PrimaryColor {
                Red,
                Yellow,
                Blue,
        }

        /// The Secondary colors according to the RYB color model.
        pub enum SecondaryColor {
                Orange,
                Green,
                Purple,
        }
}

pub mod utils {
        use super::kinds::*;

        /// Combines two primary colors in equal amounts to create a secondary color
        pub fn mix(c1: PrimaryColor, c2: SecondaryColor) -> SecondaryColor {
                // --snip--
                c2
        }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
