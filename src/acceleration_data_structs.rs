//! Data structures for acceleration vectors and easy unit conversion.

use crate::properties::gravity_coefficient;

#[derive(Clone, Copy)]
/// Single acceleration value.
pub struct Acceleration {
    /// Discretized measure of acceleration adjusted for [`crate::properties::resolution`].
    pub value: i16,
}

impl Acceleration {
    pub(crate) fn new(value: i16) -> Self {
        Acceleration { value }
    }

    /// Returns the `Acceleration` as an array of bytes in the format `[acceleration_upper, acceleration_lower]`.
    #[inline(always)]
    pub fn to_be_bytes(&self) -> [u8; 2] {
        self.value.to_be_bytes()
    }

    /// Converts acceleration from resolution adjusted i16 to units of gravity.
    pub fn as_g<G: gravity_coefficient::Property>(&self) -> f32 {
        (self.value as f32) * G::GRAVITY_COEFFICIENT
    }
}
#[derive(Clone, Copy)]
/// 3-axis acceleration vector.
pub struct AccelerationVector {
    pub x: Acceleration,
    pub y: Acceleration,
    pub z: Acceleration,
}

impl AccelerationVector {
    /// Returns the `AccelerationVector` as an array of bytes in the format `[x_upper, x_lower, y_upper, y_lower, z_upper, z_lower]`.
    #[inline(always)]
    pub fn to_be_bytes(&self) -> [u8; 6] {
        let AccelerationVector {
            x: a_x,
            y: a_y,
            z: a_z,
        } = self;

        let [[a_x_bytes_upper, a_x_bytes_lower], [a_y_bytes_upper, a_y_bytes_lower], [a_z_bytes_upper, a_z_bytes_lower]] =
            [a_x, a_y, a_z].map(|a| a.to_be_bytes());

        [
            a_x_bytes_upper,
            a_x_bytes_lower,
            a_y_bytes_upper,
            a_y_bytes_lower,
            a_z_bytes_upper,
            a_z_bytes_lower,
        ]
    }
}

pub const ZERO_ACCELERATION_VECTOR: AccelerationVector = AccelerationVector {
    x: Acceleration { value: 0 },
    y: Acceleration { value: 0 },
    z: Acceleration { value: 0 },
};
