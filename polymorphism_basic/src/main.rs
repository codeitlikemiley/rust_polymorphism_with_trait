struct Sedan;

impl Sedan {
    fn drive(&self) {
        println!("Sedan driving");
    }
}


struct SUV;

impl SUV {
    fn drive(&self) {
        println!("SUV driving");
    }
}

fn road_trip(car: &Sedan) {
        car.drive();
}

fn road_trip_suv(car: &SUV) {
        car.drive();
}


fn main() {
    let  sedan = Sedan;
    let  suv = SUV;
    road_trip(&sedan);
    road_trip_suv(&suv);
}
