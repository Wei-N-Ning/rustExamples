//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin;
//$(which mkdir) -p ${dst};
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

mod hash_map_fib {
    // put the use statement here because HashMap is only used
    // inside this module scope
    use std::collections::HashMap;

    fn fib(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
        match n {
            0 | 1 => 1,
            n => {
                if map.contains_key(&n) {
                    *map.get(&n).unwrap()
                } else {
                    let val = fib(n - 1, map) + fib(n - 2, map);
                    map.insert(n, val);
                    val
                }
            }
        }
    }

    pub fn demo() {
        let mut hm = HashMap::new();
        (35..37).for_each(|x| println!("{} -> {}", x, fib(x, &mut hm)))
    }
}

fn main() {
    hash_map_fib::demo();
}
