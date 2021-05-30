pub enum UnitType {
    Length,
    Area,
    Mass,
}

struct Unit {
    utype: UnitType,
    name: String,
    factor: f32,
}
