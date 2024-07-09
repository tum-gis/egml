use crate::error::Error;

// TODO: implement ordering and comparisons
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DirectPosition {
    x: f64,
    y: f64,
    z: f64,
}

impl DirectPosition {
    pub fn new(x: f64, y: f64, z: f64) -> Result<Self, Error> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return Err(Error::ValueNotFinite(""));
        }

        Ok(Self { x, y, z })
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn coords(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }

    pub fn set_x(&mut self, val: f64) -> Result<(), Error> {
        if !val.is_finite() {
            return Err(Error::ValueNotFinite("x"));
        }
        self.x = val;
        Ok(())
    }

    pub fn set_y(&mut self, val: f64) -> Result<(), Error> {
        if !val.is_finite() {
            return Err(Error::ValueNotFinite("y"));
        }
        self.y = val;
        Ok(())
    }

    pub fn set_z(&mut self, val: f64) -> Result<(), Error> {
        if !val.is_finite() {
            return Err(Error::ValueNotFinite("z"));
        }
        self.z = val;
        Ok(())
    }

    pub const MIN: DirectPosition = DirectPosition {
        x: f64::MIN,
        y: f64::MIN,
        z: f64::MIN,
    };
    pub const MAX: DirectPosition = DirectPosition {
        x: f64::MAX,
        y: f64::MAX,
        z: f64::MAX,
    };
}

impl From<DirectPosition> for nalgebra::Vector3<f64> {
    fn from(item: DirectPosition) -> Self {
        Self::new(item.x, item.y, item.z)
    }
}

impl From<nalgebra::Vector3<f64>> for DirectPosition {
    fn from(item: nalgebra::Vector3<f64>) -> Self {
        Self::new(item.x, item.y, item.z).unwrap()
    }
}

impl From<DirectPosition> for nalgebra::Point3<f64> {
    fn from(item: DirectPosition) -> Self {
        Self::new(item.x, item.y, item.z)
    }
}

impl From<DirectPosition> for nalgebra::Point3<f32> {
    fn from(item: DirectPosition) -> Self {
        Self::new(item.x as f32, item.y as f32, item.z as f32)
    }
}

impl From<nalgebra::Point3<f64>> for DirectPosition {
    fn from(item: nalgebra::Point3<f64>) -> Self {
        // TODO: how to handle error?
        Self::new(item.x, item.y, item.z).unwrap()
    }
}
