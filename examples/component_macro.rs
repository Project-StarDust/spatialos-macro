use spatialos_macro::spatial_component;

#[spatial_component(404)]
struct Mass {
    #[ID(1)] mass: f64,
    #[ID(2)] thrust: f32,
}

fn main() {}
