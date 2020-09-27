use crate::algebra::Vector3;
use std::convert::{TryFrom, TryInto};
use std::ops::{Add, Mul, Neg};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, Ord, PartialOrd)]
pub enum UnitVector {
    Right,
    Left,
    Up,
    Down,
    Front,
    Back,
}

impl UnitVector {
    pub fn to_vector(&self) -> Vector3 {
        (*self).into()
    }

    pub fn rotate(&self, axis: &UnitVector) -> UnitVector {
        let vector: Vector3 = (*self).to_vector();
        vector.rotate(axis).try_into().unwrap()
    }

    pub fn cross(&self, other: &UnitVector) -> UnitVector {
        let vector = (*self).to_vector();
        let other = (*other).to_vector();
        vector.cross(&other).try_into().unwrap()
    }

    pub fn abscissa_and_ordinate(&self) -> (UnitVector, UnitVector) {
        use UnitVector::*;

        match self {
            Right => (Up, Front),
            Up => (Front, Right),
            Front => (Right, Up),
            Left => (Back, Down),
            Down => (Left, Back),
            Back => (Down, Left),
        }
    }
}

impl TryFrom<Vector3> for UnitVector {
    type Error = ();

    fn try_from(value: Vector3) -> Result<Self, Self::Error> {
        use UnitVector::*;

        match (value.x, value.y, value.z) {
            (1, 0, 0) => Ok(Right),
            (-1, 0, 0) => Ok(Left),
            (0, 1, 0) => Ok(Up),
            (0, -1, 0) => Ok(Down),
            (0, 0, 1) => Ok(Front),
            (0, 0, -1) => Ok(Back),
            (_, _, _) => Err(()),
        }
    }
}

impl Add<UnitVector> for UnitVector {
    type Output = Vector3;

    fn add(self, other: UnitVector) -> Vector3 {
        let self_vector = self.to_vector();
        let other_vector = other.to_vector();
        self_vector + other_vector
    }
}

impl Mul<isize> for UnitVector {
    type Output = Vector3;

    fn mul(self, other: isize) -> Vector3 {
        let vector = self.to_vector();
        other * vector
    }
}

impl Mul<UnitVector> for isize {
    type Output = Vector3;

    fn mul(self, other: UnitVector) -> Vector3 {
        let vector = other.to_vector();
        self * vector
    }
}

impl Neg for UnitVector {
    type Output = UnitVector;

    fn neg(self) -> Self::Output {
        use UnitVector::*;

        match self {
            Right => Left,
            Left => Right,
            Up => Down,
            Down => Up,
            Front => Back,
            Back => Front,
        }
    }
}
