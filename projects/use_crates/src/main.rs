// to compile and link the rand crate
extern crate rand; 

/*
[package]
name = "use_crates"
version = "0.1.0"
authors = ["Wei Ning <wei@canva.com>"]
edition = "2018"

[dependencies]
# 1. 
# use the bleeding edge version from repo
# or it could be an in-house project
# rand = { git = "https://github.com/rust-random/rand.git" }

# 2.
# use a local folder
# doomlib = { path = "../doomlib" }

# 3. use a public version
rand = "0.5.5"
*/

/*
.
├── Cargo.lock  <-- written out after cargo resolves the deps; act like cache
├── Cargo.toml
├── src
│   └── main.rs
└── target  <-- artifacts
    └── debug
        ├── build
        │   ├── libc-7d287d5279173255
        │   │   ├── out
        │   │   ├── output
        │   │   ├── root-output
        │   │   └── stderr
        │   └── libc-fa8277e1f8db45c2
        │       ├── build-script-build
        │       ├── build_script_build-fa8277e1f8db45c2
        │       ├── build_script_build-fa8277e1f8db45c2.d
        │       └── build_script_build-fa8277e1f8db45c2.dSYM
        │           └── Contents
        │               ├── Info.plist
        │               └── Resources
        │                   └── DWARF
        │                       └── build_script_build-fa8277e1f8db45c2
        ├── deps
        │   ├── libc-2f40e99eb2f697ee.d
        │   ├── liblibc-2f40e99eb2f697ee.rlib
        │   ├── librand-6f2bc656f93dccda.rlib
        │   ├── librand_core-57de88e03b8419ee.rlib
        │   ├── librand_core-d1021efdde25f32d.rlib
        │   ├── rand-6f2bc656f93dccda.d
        │   ├── rand_core-57de88e03b8419ee.d
        │   ├── rand_core-d1021efdde25f32d.d
        │   ├── use_crates-b4afdf345a6ae5b2
        │   ├── use_crates-b4afdf345a6ae5b2.d
        │   └── use_crates-b4afdf345a6ae5b2.dSYM
        │       └── Contents
        │           ├── Info.plist
        │           └── Resources
        │               └── DWARF
        │                   └── use_crates-b4afdf345a6ae5b2
        ├── examples
        ├── incremental
        │   └── use_crates-102brou785azv
        │       ├── s-faxmdu8va6-1fte0uw-2id2ct4ednaam
        │       │   ├── 1ta1dtut5kwe5kx5.o
        │       │   ├── 2a9arm3x2cuqdr1i.o
        │       │   ├── 30qvt3aj6hr3nh5m.o
        │       │   ├── 3lsgmwfbe7u9yx6.o
        │       │   ├── 3rr9ffpw61qgu6tn.o
        │       │   ├── 3sbjica1i3rmt3mb.o
        │       │   ├── 3yk4vv7dgqvwoep4.o
        │       │   ├── 4erd6n3qbxl7tz67.o
        │       │   ├── 4u5n1pcch7hzdt12.o
        │       │   ├── 57b66jitp5ng2c69.o
        │       │   ├── dep-graph.bin
        │       │   ├── query-cache.bin
        │       │   ├── work-products.bin
        │       │   └── yxymq29cyibhd2q.o
        │       └── s-faxmdu8va6-1fte0uw.lock
        ├── native
        ├── use_crates
        ├── use_crates.d
        └── use_crates.dSYM -> deps/use_crates-b4afdf345a6ae5b2.dSYM
*/

extern crate examplelib;

// recall C++ using namespace x; and using namespace::sym;
use rand::random;

fn main() {
    let x: u8 = random();  // rand::random
    println!("{}", x);
}
