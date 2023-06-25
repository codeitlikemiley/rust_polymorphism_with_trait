trait LandCapable {
    // default implementation for drive on LandCapable Trait
    fn drive(&self) {
        println!("Default Drive");
    }
}

trait WaterCapable {
    // default implementation for sail on WaterCapable Trait
    fn sail(&self) {
        println!("Default Sail");
    }
}


// super trait , we combine two traits into one
trait Amphibious: LandCapable + WaterCapable {}

struct Hovercraft;

// we can implement the super trait for Hovercraft
impl Amphibious for Hovercraft {}

// we can implement the sub traits for Hovercraft
impl LandCapable for Hovercraft {}
impl WaterCapable for Hovercraft {}

// we can use the super trait as a parameter
fn hover (craft: &impl Amphibious) {
    craft.drive();
    craft.sail();
}

fn main() {
    let hovercraft = Hovercraft;
    println!("Hovercraft implementing super trait Amphibious");
    hover(&hovercraft);
}

