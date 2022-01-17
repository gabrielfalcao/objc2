#![cfg(feature = "derive")]
use objc2_encode::{Encode, Encoding, RefEncode};

#[cfg(target_pointer_width = "32")]
type CGFloat = f32;

#[cfg(target_pointer_width = "64")]
type CGFloat = f64;

#[derive(Encode, RefEncode)]
#[repr(C)]
struct CGPoint {
    x: CGFloat,
    y: CGFloat,
}

#[test]
fn cgpoint() {
    let enc = Encoding::Struct("CGPoint", &[CGFloat::ENCODING, CGFloat::ENCODING]);
    assert_eq!(CGPoint::ENCODING, enc);
    assert_eq!(CGPoint::ENCODING_REF, Encoding::Pointer(&enc));
}
