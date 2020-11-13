#[macro_use]
extern crate spatialos_macro;
#[derive(SpatialComponent)]
#[id(404)]
struct Mass {
    #[field_id(1)]
    mass: f64,
    #[field_id(2)]
    thrust: f32,
}

#[derive(SpatialType)]
struct Coordinates {
    #[field_id(1)]
    x: f64,
    #[field_id(2)]
    y: f64,
    #[field_id(3)]
    z: f64,
}
#[derive(SpatialComponent)]
#[id(54)]
struct Position {
    #[field_id(1)]
    coords: Coordinates,
}

fn main() {}
