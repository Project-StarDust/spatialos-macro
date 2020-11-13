#[macro_use]
extern crate spatialos_macro;


#[derive(SpatialComponent)]
#[id(404)]
struct Mass {
    #[field_id(1)] mass: f64,
    #[field_id(2)] thrust: f32,
}

fn main() {}
