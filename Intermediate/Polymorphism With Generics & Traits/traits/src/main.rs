
trait Vehicle: Paint {
    fn park(&self);
    fn get_default_color() -> String{
        "black".to_owned()
    }
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16

}

struct Car {
    info: VehicleInfo,
}

impl Car {
    fn new(make: &str, model: &str, year: u16) -> Car {
        let info = VehicleInfo {
            make: make.to_owned(),
            model: model.to_owned(),
            year,
        };
        
        Car {info}
}
}
impl Vehicle for Car {
    fn park(&self) {
        println!("parking car.");
    }
}
impl Paint for Car {}

struct Truck {
    info: VehicleInfo,
}



impl Truck {
    fn unload(&self) {
        println!("unloading truck.");
    }

    fn new(make: &str, model: &str, year: u16) -> Truck {
        let info = VehicleInfo {
            make: make.to_owned(),
            model: model.to_owned(),
            year,
        };
        
        Truck {info}
}
}

impl Vehicle for Truck {
    fn park(&self) {
        println!("parking truck.");
    }
}
impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {}" , color);
    }
}

fn main() {

    let car = Car::new("honda", "Civic", 1995);
    let house = House{};
    let object = create_paintable_object(true);

    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

    paint_red(&car);
    paint_red(&house);
    paint_red(object.as_ref());

     //paint_vehicle_red(&object);
     paint_vehicle_red(&car);
     //paint_vehicle_red(&house);

    
}

//one way to define trait bound.
// fn paint_red<T: Paint>(object: &T) {
//     object.paint("red".to_owned());
// }
// rewriting to accept a trait object instead of a generic type
fn paint_red(object: &dyn Paint) {
    object.paint("red".to_owned());
}


//another way to define trait bound.
//says object must be a reference to 
//something that impliments the Paint trait.
fn paint_red2(object: &impl Paint) {
    object.paint("red".to_owned());
}

//In this way of defining a trait bound, you may add more traits with a '+'
fn paint_vehicle_red<T>(object: &T) where T: Vehicle {
    object.paint("red".to_owned());
}

/*/trait bound used as a return type
fn create_paintable_object() -> impl Paint {
    House{}
    //if we were to return different or more concrete types than just these, we'd need a trait object
}
*/

//this is how to do dynamic dispatching. 
//the box dyn Trait is a dynamic (dyn) trait object. trait objects need to be behind a pointer which is what the Box is
// Box is a smart pointer that points to something alloocated on the heap 
// because the paint method is tied to different struct types (house and car), if we want to return more than one on 
// this function that uses a genaric type, the compiler needs to know which method to call depending on the returned type.
// if it was just one type that is returned, the compiler would know to return that one type with that assigned paint method. 
// but bcause we have options of two concrete type, then we need a pointer to assign the correct type and version of the method. 

fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    match vehicle {
        true => Box::new(Car::new("hoho", "HHe", 1990)),
        false => Box::new(House{})
    }
    // if vehicle {
    //     Box::new(Car::new("honda", "Civic", 1995))
    // } else {
    //     Box::new(House{})
    // }
}