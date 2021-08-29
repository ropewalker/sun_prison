mod unit_vector;

use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
pub use unit_vector::*;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, Ord, PartialOrd)]
pub struct Vector3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Vector3 {
    pub fn manhattan_length(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: &Vector3) -> isize {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn rotate(&self, axis: &UnitVector) -> Vector3 {
        use UnitVector::*;

        let matrix = match axis {
            Right => Matrix3 {
                x: Vector3 { x: 1, y: 0, z: 0 },
                y: Vector3 { x: 0, y: 0, z: 1 },
                z: Vector3 { x: 0, y: -1, z: 0 },
            },
            Up => Matrix3 {
                x: Vector3 { x: 0, y: 0, z: -1 },
                y: Vector3 { x: 0, y: 1, z: 0 },
                z: Vector3 { x: 1, y: 0, z: 0 },
            },
            Front => Matrix3 {
                x: Vector3 { x: 0, y: 1, z: 0 },
                y: Vector3 { x: -1, y: 0, z: 0 },
                z: Vector3 { x: 0, y: 0, z: 1 },
            },
            Left => Matrix3 {
                x: Vector3 { x: 1, y: 0, z: 0 },
                y: Vector3 { x: 0, y: 0, z: -1 },
                z: Vector3 { x: 0, y: 1, z: 0 },
            },
            Down => Matrix3 {
                x: Vector3 { x: 0, y: 0, z: 1 },
                y: Vector3 { x: 0, y: 1, z: 0 },
                z: Vector3 { x: -1, y: 0, z: 0 },
            },
            Back => Matrix3 {
                x: Vector3 { x: 0, y: -1, z: 0 },
                y: Vector3 { x: 1, y: 0, z: 0 },
                z: Vector3 { x: 0, y: 0, z: 1 },
            },
        };

        matrix * self
    }
}

impl From<(isize, isize, isize)> for Vector3 {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Vector3 { x, y, z }
    }
}

impl From<UnitVector> for Vector3 {
    fn from(unit_vector: UnitVector) -> Self {
        use UnitVector::*;

        match unit_vector {
            Right => (1, 0, 0).into(),
            Left => (-1, 0, 0).into(),
            Up => (0, 1, 0).into(),
            Down => (0, -1, 0).into(),
            Front => (0, 0, 1).into(),
            Back => (0, 0, -1).into(),
        }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<UnitVector> for Vector3 {
    type Output = Vector3;

    fn add(self, other: UnitVector) -> Vector3 {
        let vector: Vector3 = other.as_vector();
        self + vector
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<isize> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: isize) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector3> for isize {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

pub struct Matrix3 {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
}

impl Mul<&Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x.x * other.x + self.y.x * other.y + self.z.x * other.z,
            y: self.x.y * other.x + self.y.y * other.y + self.z.y * other.z,
            z: self.x.z * other.x + self.y.z * other.y + self.z.z * other.z,
        }
    }
}
