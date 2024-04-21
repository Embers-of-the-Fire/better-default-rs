pub trait ConstDefault {
    const DEFAULT: Self;
}

/// Re-exporting `Default` to prevent naming pollution.
pub use core::default::Default;
