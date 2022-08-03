use crate::types::Triangle;

// Processes triangles
pub trait GeometryShader<const GSIN: usize, const GSOUT: usize> {
    fn run(triangle: Triangle<GSIN>) -> Triangle<GSOUT>;
}

pub struct DefaultGeometryShader;

impl<const GS: usize> GeometryShader<GS, GS> for DefaultGeometryShader {
    fn run(triangle: Triangle<GS>) -> Triangle<GS> {
        triangle
    }
}
