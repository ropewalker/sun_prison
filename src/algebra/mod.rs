use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Vector3 {
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

    pub fn rotate(&self, axis: &Vector3) -> Vector3 {
        let matrix = match (axis.x, axis.y, axis.z) {
            (1, 0, 0) => Matrix3 {
                x: Vector3 { x: 1, y: 0, z: 0 },
                y: Vector3 { x: 0, y: 0, z: 1 },
                z: Vector3 { x: 0, y: -1, z: 0 },
            },
            (0, 1, 0) => Matrix3 {
                x: Vector3 { x: 0, y: 0, z: -1 },
                y: Vector3 { x: 0, y: 1, z: 0 },
                z: Vector3 { x: 1, y: 0, z: 0 },
            },
            (0, 0, 1) => Matrix3 {
                x: Vector3 { x: 0, y: 1, z: 0 },
                y: Vector3 { x: -1, y: 0, z: 0 },
                z: Vector3 { x: 0, y: 0, z: 1 },
            },
            (-1, 0, 0) => Matrix3 {
                x: Vector3 { x: 1, y: 0, z: 0 },
                y: Vector3 { x: 0, y: 0, z: -1 },
                z: Vector3 { x: 0, y: 1, z: 0 },
            },
            (0, -1, 0) => Matrix3 {
                x: Vector3 { x: 0, y: 0, z: 1 },
                y: Vector3 { x: 0, y: 1, z: 0 },
                z: Vector3 { x: -1, y: 0, z: 0 },
            },
            (0, 0, -1) => Matrix3 {
                x: Vector3 { x: 0, y: -1, z: 0 },
                y: Vector3 { x: 1, y: 0, z: 0 },
                z: Vector3 { x: 0, y: 0, z: 1 },
            },
            _ => panic!("wrong axis!"),
        };

        matrix * self
    }
}

impl From<(isize, isize, isize)> for Vector3 {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Vector3 { x, y, z }
    }
}

impl<'a, 'b> Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<'a, 'b> Sub<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign<&Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: &Vector3) {
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

pub struct RotationInfo {
    pub axis: Vector3,
    pub layer: isize,
}
