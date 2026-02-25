#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SurfaceInterpolation {
    None,
    Planar,
    Spherical,
    Elliptical,
    Conic,
    Tin,
    ParametricCurve,
    PolynomialSpline,
    RationalSpline,
    TriangulatedSpline,
}
