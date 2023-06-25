trait LandCapable {
    // default implementation for drive on LandCapable Trait
    fn drive(&self) {
        println!("Default Drive");
    }
}

struct Sedan;

impl LandCapable for Sedan {}

struct SUV;

impl LandCapable for SUV {}

fn road_trip(car: &impl LandCapable) {
    car.drive();
}

fn main() {
    let sedan = Sedan;
    let suv = SUV;
    road_trip(&sedan);
    road_trip(&suv);
}
