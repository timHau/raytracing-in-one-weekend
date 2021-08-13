#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    data: [f64; 3],
}

impl Vec3 {
    pub(crate) fn new(data: [f64; 3]) -> Self {
        Self { data }
    }

    pub(crate) fn x(&self) -> f64 {
        self.data[0]
    }

    pub(crate) fn y(&self) -> f64 {
        self.data[1]
    }

    pub(crate) fn z(&self) -> f64 {
        self.data[2]
    }

    pub(crate) fn len(&self) -> f64 {
        self.len_square().sqrt()
    }

    pub(crate) fn len_square(&self) -> f64 {
        self.dot(self)
    }

    pub(crate) fn dot(&self, other: &Self) -> f64 {
        self.data[0] * other.data[0] + self.data[1] * other.data[1] + self.data[2] * other.data[2]
    }

    pub(crate) fn cross(&self, other: &Self) -> Vec3 {
        Vec3 {
            data: [
                self.data[1] * other.data[2] - self.data[2] * other.data[1],
                self.data[2] * other.data[0] - self.data[0] * other.data[2],
                self.data[0] * other.data[1] - self.data[1] * other.data[0],
            ],
        }
    }

    pub(crate) fn as_unit_vec(self) -> Self {
        let len = self.len();
        self / len
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self.data[0] += other.data[0];
        self.data[1] += other.data[1];
        self.data[2] += other.data[2];
        self
    }
}

impl std::ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
            ],
        }
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            data: [
                self.data[0] - other.data[0],
                self.data[1] - other.data[1],
                self.data[2] - other.data[2],
            ],
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.data[0] = -self.data[0];
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
        self
    }
}

impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec3 {
            data: [
                self.data[0] * scalar,
                self.data[1] * scalar,
                self.data[2] * scalar,
            ],
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(mut self, scalar: f64) -> Self::Output {
        &self * scalar
    }
}

impl std::ops::Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            data: [
                self.data[0] * other.data[0],
                self.data[1] * other.data[1],
                self.data[2] * other.data[2],
            ],
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(mut self, scalar: f64) -> Vec3 {
        assert_ne!(scalar, 0.0);
        Vec3 {
            data: [
                self.data[0] / scalar,
                self.data[1] / scalar,
                self.data[2] / scalar,
            ],
        }
    }
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr ) => {
        $crate::vec3::Vec3::new([$x, $y, $z])
    };
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn access() {
        let v = Vec3::new([1.0, 2.0, 3.0]);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn len() {
        let v = Vec3::new([1.0, 2.0, 3.0]);
        assert_eq!(
            v.len(),
            (f64::powf(1.0, 2.0) + f64::powf(2.0, 2.0) + f64::powf(3.0, 2.0)).sqrt()
        )
    }

    #[test]
    fn len_square() {
        let v = Vec3::new([1.0, 2.0, 3.0]);
        assert_eq!(v.len_square(), 14.0)
    }

    #[test]
    fn add_mut() {
        let v = Vec3::new([1.0, 1.0, 0.0]);
        let u = Vec3::new([2.0, 1.0, 0.0]);
        assert_eq!(v + u, Vec3::new([3.0, 2.0, 0.0]));
    }

    #[test]
    fn add_immut() {
        let v = &Vec3::new([1.0, 1.0, 0.0]);
        let u = &Vec3::new([2.0, 1.0, 0.0]);
        assert_eq!(v + u, Vec3::new([3.0, 2.0, 0.0]));
    }

    #[test]
    fn mul_scalar() {
        let v = &Vec3::new([1.0, 2.5, 0.0]);
        assert_eq!(v * 4.0, Vec3::new([4.0, 10.0, 0.0]));
    }

    #[test]
    fn div() {
        let v = Vec3::new([2.0, 4.0, 5.0]) / 2.0;
        assert_eq!(v, Vec3::new([1.0, 2.0, 2.5]));
    }

    #[test]
    fn neg() {
        let v = Vec3::new([1.0, 2.0, 3.0]);
        assert_eq!(-v, Vec3::new([-1.0, -2.0, -3.0]));
    }

    #[test]
    fn dot() {
        let v = &Vec3::new([1.0, 2.0, 3.0]);
        let u = &Vec3::new([1.0, 2.0, 3.0]);
        assert_eq!(v.dot(u), 14.0);
    }

    #[test]
    fn cross() {
        let v = &Vec3::new([1.0, 2.0, 3.0]);
        let u = Vec3::new([3.0, 1.0, -1.0]);
        assert_eq!(v.cross(&u), Vec3::new([-5.0, 10.0, -5.0]));
    }

    #[test]
    fn unit_vec() {
        let v = Vec3::new([4.3, 9.5, 12343.3]).as_unit_vec();
        assert_eq!(v.len(), 1.0);
    }
}
