use wcore::time::Time;

#[derive(Clone, Default)]
pub struct TaikoCircle {
    pub time  : Time,
    
    pub big   : bool,
    pub color : TaikoColor,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TaikoColor {
    #[default]
    KAT,
    DON,
}

impl TaikoColor {
    pub fn toggle(&mut self) {
        match self {
            TaikoColor::KAT => *self = TaikoColor::DON,
            TaikoColor::DON => *self = TaikoColor::KAT,
        }
    }
}