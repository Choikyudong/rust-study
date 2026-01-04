//! 해당 주석을 이용하여
//! 모듈 전체에 대한 문서를 작성한다.
//! 관례상 src/lib.rs에 작성한다.

pub use self::kinds::PrimaryColor;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}
