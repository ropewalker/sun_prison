use crate::algebra::Vector3;
use std::convert::{TryFrom, TryInto};
use std::ops::{Add, Neg};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum UnitVector {
    Right,
    Left,
    Up,
    Down,
    Front,
    Back,
}

impl UnitVector {
    pub fn rotate(&self, axis: &UnitVector) -> UnitVector {
        let vector: Vector3 = self.into();
        vector.rotate(axis).try_into().unwrap()
    }

    pub fn cross(&self, other: &UnitVector) -> UnitVector {
        let vector: Vector3 = self.into();
        let other: Vector3 = other.into();
        vector.cross(&other).try_into().unwrap()
    }
}

impl TryFrom<&Vector3> for UnitVector {
    type Error = ();

    fn try_from(value: &Vector3) -> Result<Self, Self::Error> {
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

impl TryFrom<Vector3> for UnitVector {
    type Error = ();

    fn try_from(value: Vector3) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl<'a, 'b> Add<&'b UnitVector> for &'a UnitVector {
    type Output = Vector3;

    fn add(self, other: &UnitVector) -> Vector3 {
        let self_vector: Vector3 = self.into();
        let other_vector: Vector3 = other.into();
        &self_vector + &other_vector
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
