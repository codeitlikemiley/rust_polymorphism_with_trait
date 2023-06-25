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

// using impl trait is static dispatch
 fn road_trip(car: &impl LandCapable) {
        car.drive();
}


fn main() {
    let  sedan = Sedan;
    let  suv = SUV;
    road_trip(&sedan);
    road_trip(&suv);
}
