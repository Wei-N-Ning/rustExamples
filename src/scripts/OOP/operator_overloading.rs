//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

use std::cmp::PartialEq;
use std::ops::Add;

// PartialEq and Eq
// Equality:

// ---- Eq ----
    // ---- PartialEq ----
    // Symmetic: a == b -> b == a
    // Transitive: a == b and b == c -> a == c
// Reflexive: a == a

// Types that can not implement full Eq include floats:
// it does not make sense to compare NAN with NAN, so 
// this is illegal NAN != NAN for f32

// other operators:
// x == y: PartialEq/Eq; used by Vec::contains()
// x < y: PartialOrd / Ord; used by [T]::sort()
// x + y: Add; used by std::time::SystemTime
// x - y: Subtraction
// -x: Negative
// x * y: Multiplication
// x / y: Division
// x % y: Remainder
// !x: Not
// x & y: BitAnd
// x | y: BitOr
// x ^ y: BitXor
// x << y: Shl
// x >> y: Shr

// special `drop` operator: to implement resource-releasing
// mechanism (dtor)

// container[index]: Index/IndexMut, used by std::opts::Range
// (): Fn/FnMut/FnOnce
// *: Deref/DerefMut

struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    // I can not name them lhs, rhs ('self' has special meaning
    // to the compiler); stick to 'self' and 'other' convention
    fn eq(&self, other: &Point) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

fn test_point_equal() {
    let p1 = Point { x: 10, y: 10 };
    let p2 = Point { x: 10, y: 10 };

    // == and != invokes PartialEq trait/operator
    println!("{}", p1 == p2);
    println!("{}", p1 != p2);
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Vector {
    x: i32,
    y: i32,
}

fn test_vector_addition() {
    let v1 = Vector { x: 10, y: 10 };
    let v2 = Vector { x: 1, y: 2};
    let v3 = v1 + v2;
    // v3 += v2;  requires std::ops::AddAssign
    println!("{} {}", v3.x, v3.y);
}

fn main() {
    test_point_equal();
    test_vector_addition();
}
