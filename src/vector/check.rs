impl super::V2 {
    pub fn is_null(&self) -> bool {
        self.x() == 0.0 && self.y() == 0.0
    }

    pub fn is_horizontal(&self) -> bool {
        self.x() != 0.0 && self.y() == 0.0
    }

    pub fn is_vertical(&self) -> bool{
        self.x() == 0.0 && self.y() != 0.0
    }
}