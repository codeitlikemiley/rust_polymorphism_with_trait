trait LandCapable {
    fn drive(&self);
}

struct Sedan;

impl LandCapable for Sedan {
    fn drive(&self) {
        println!("Sedan driving");
    }
}

struct SUV;

impl LandCapable for SUV {
    fn drive(&self) {
        println!("SUV driving");
    }
}

// using dynamic dispatch with dyn keyword
// has a runtime cost
fn road_trip(car: &dyn LandCapable) {
        car.drive();
}


fn main() {
    let  sedan = Sedan;
    let  suv = SUV;
    road_trip(&sedan);
    road_trip(&suv);
}
