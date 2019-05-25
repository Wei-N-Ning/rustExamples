//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// anything can be a "Vehicle" if it implements these 
// three methods
trait Vehicle {
    // static method
    // rust's convention for constructing an object
    fn new(name: &'static str) -> Self;
    
    // instance method
    fn move_(&self) -> ();
    fn to_string(&self) {
        // provide a default impl
        println!("vehicle");
    }  
}

struct Airplane {
    name: &'static str,
}

impl Vehicle for Airplane {
    fn new(name: &'static str) -> Self {
        Airplane {name: name}
    }
    fn move_(&self) {
        ;
    }
    fn to_string(&self) {
        println!("Airplane({})", self.name);
    }
}

// use trait bound (must implement Vehicle trait)
fn repair<T: Vehicle>(vehicle: T) {
    vehicle.move_();
    vehicle.to_string();
}

fn main() {
    let plane = Airplane::new("f15");
    repair(plane);
}
