#![cfg(feature = "internal_enhanced_float")]
mod powf {
    //micromath's F32Ext is drop-in compatible with std floating point operations. However, we prefer
    //libm over micromath and std over libm, so this function definition is enabled if either std is
    //available or both micromath is available and libm is not.
    #[cfg(any(feature = "std", all(feature = "micromath", not(feature = "libm"))))]
    #[inline]
    pub fn powf(x: f32, y: f32) -> f32 {
        x.powf(y)
    }
    #[cfg(all(feature = "libm", not(feature = "std")))]
    pub use libm::powf;
    #[cfg(all(feature = "micromath", not(feature = "std"), not(feature = "libm")))]
    pub use micromath::F32Ext;
}
pub use powf::*;
#[cfg(feature = "error_propagation")]
mod sqrt {
    #[cfg(any(feature = "std", all(feature = "micromath", not(feature = "libm"))))]
    pub fn sqrt(x: f32) -> f32 {
        x.sqrt()
    }
    #[cfg(all(feature = "libm", not(feature = "std")))]
    pub fn sqrt(x: f32) -> f32 {
        libm::sqrtf(x)
    }
    #[cfg(all(feature = "micromath", not(feature = "std"), not(feature = "libm")))]
    pub use micromath::F32Ext;
}
#[cfg(feature = "error_propagation")]
pub use sqrt::*;