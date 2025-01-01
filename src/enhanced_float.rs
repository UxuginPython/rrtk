#![cfg(feature = "internal_enhanced_float")]
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
//There is a strange behavior where without F32Ext, the compiler, of course, errors with only the
//micromath feature enabled. However, with F32Ext, testing with the same configuration yields an
//unused import warning although the trait does seem to be necessary. Interestingly, this behavior
//only occurs when testing and not with building normally. The cfg_attr is a workaround to prevent
//the warning.
#[cfg(all(feature = "micromath", not(feature = "std"), not(feature = "libm")))]
#[cfg_attr(test, allow(unused_imports))]
pub use micromath::F32Ext;
