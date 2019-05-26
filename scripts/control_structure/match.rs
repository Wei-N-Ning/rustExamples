//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

enum Action {
    Drive,
    Turn(Direction),
    Stop,
    _Undefined,
}

enum Direction {
    Left,
    Right,
}

fn process_action(ac: &Action) {
    // non-exhaustive patterns (compiler knows all the possible types
    // of action)
    // match ac {}

    // to save some typing:
    // use self::Direction::*
    // use self::Action::*
    // recall ruby/crystal also has the concept of "self"

    match ac {
        // Action::Drive is, by itself, a value
        // however Action::Turn is not a value unless instantiated 
        // with a direction enum
        Action::Drive => {
            println!("dr");
        },
        
        // this can do but Turn(dir) is better
        // Action::Turn(Direction::Left) => {
        //     println!("turn(left)");
        // },
        // Action::Turn(Direction::Right) => {
        //     println!("turn(right)");
        // },

        Action::Turn(dir) => {
            println!("turn({})", match dir {
                Direction::Right => "r",
                Direction::Left => "l",
            });
        }
        Action::Stop => {
            println!("stop");
        },
        _ => {}
    }
}

fn main() {
    [
        Action::Drive, 
        Action::Turn(Direction::Right), 
        Action::Stop, 
        Action::Drive, 
        Action::Turn(Direction::Left)
    ].iter().for_each( |ac| process_action(&ac) );
}