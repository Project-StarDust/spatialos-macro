use spatial_macro::spatial_component;

#[spatial_component(404)]
struct Mass {
    mass: f64,
    thrust: f32,
}

fn main() {
    let mass = Mass::new(0.2, 0.0);
    println!("{:?}", mass);
}
