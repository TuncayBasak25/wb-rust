mod check;
mod chain_op;
mod angle;

#[derive(Clone, Copy, Debug, Default)]
pub struct V2(f32, f32);

impl V2 {
    pub fn x(&self) -> f32{
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn mag(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2)).powf(0.5)
    }

    pub fn dir(&self) -> V2 {
        let mut unit = self.clone();
        unit.normalize();
        unit
    }
}