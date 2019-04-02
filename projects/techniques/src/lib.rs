
pub mod utilities {

    // &Foo is a reference
    // &Foo is not possible to be invalid or Null
//    pub fn f(ptr: Option(&Foo)) {
//        // either None or Something
//        match ptr {
//            Some(ptr) => ptr.g(),
//            None => {}
//        }
//    }

    // Result object wraps the value object and the error object
    // recall Go's idiom
    // format!() is a macro not a function call
    pub fn compute(i: i32) -> Result<i32, String> {
        match i {
            i if i >= 0 => Ok(i + 10),
            _ => Err(format!("expect greater than 0"))
        }
    }

    pub fn maybe_compute(y: Option<i32>) -> Option<i32> {
//        match y {
//            Some(yy) => Some(yy + 4),
//            None => None,
//        }
        y.map(|x| x + 4)  // uses the closure form and map()
    }
}

#[cfg(test)]
mod tests {
    use super::utilities::*;

    #[test]
    fn use_match_control_flow() {
        match compute(10) {
            Ok(result) => assert_eq!(20, result),
            _ => panic!()
        }
    }

    // better code readability
    // else block is optional
    #[test]
    fn use_if_let_control_flow() {
        if let Ok(i) = compute(10) {
            assert_eq!(20, i)  // do thing with i in prod code
        } else {
            panic!()
        }
    }

    #[test]
    fn use_let_and_return_control_flow() {
        let i = match compute(10) {
            Ok(i) => i,
            _ => panic!(),  // err => return err, in prod code
        };

        // do thing with i in prod code
        assert_eq!(20, i)
    }

    // I can not do that in a test function but
    // a shorter form of the above pattern exists:
    // let i = compute(10)?;
    // and:
    // let i = h()?.foo()?.bar;
    // the error is returned to the caller so there is no
    // need to catch the error per accessor
    // recall the same pattern in C++ will look like:
    // amc->compute()->samples()->size();
    // and it crashes in all sorts of ways

    #[test]
    fn given_none_expect_none() {
        assert_eq!(None, maybe_compute(None))
    }

    #[test]
    fn demo_vec_iterator() {
        let vec = vec![0, 1, 2, 3];
        let _ = vec.iter().map(|x| x + 1);
        vec.iter().for_each(|x| println!("{}", x));
    }

    #[test]
    fn demo_vec_chain_iterator() {
        let vec = vec![0, 1, 2, 3];
        for (i, v) in vec.iter().chain(Some(42).iter()).enumerate() {
            println!("{}: {}", i, v);
        }
    }

}
