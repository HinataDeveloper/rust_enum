// main.rs
// Date: Tue Nov 03 2026
// Educational and Practice Rust Programming Language Code

// Project: Learning enum in Rust programming language
// Goal: ...
// Dependency: Without any dependency

// rustc 1.101.0-nightly (3e92e1a2d 2026-11-01)
// binary: rustc
// commit-hash: 3e92e1a2d20b991ef23a6c6064c34d57ee8c98d3
// commit-date: 2026-11-01
// host: x86_64-unknown-linux-gnu
// release: 1.101.0-nightly
// LLVM version: 23.1.5

// cargo 1.101.0-nightly (31d3246e0 2026-10-27)
// release: 1.101.0-nightly
// commit-hash: 31d3246e0e8d3a80d4a90b54aa48ff180adb408b
// commit-date: 2026-10-27
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.14-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

fn main() {
    println!("\n");

    // The definition order specify the rank of each member.
    match Daniel::Five.cmp(&Daniel::Two) {
        std::cmp::Ordering::Greater => println!("Daniel::Five is GREATER than Daniel::Two"),
        std::cmp::Ordering::Less => println!("Daniel::Five is LESS than Daniel::Two"),
        std::cmp::Ordering::Equal => println!("Daniel::Five is EQUALS Daniel::Two"),
    }

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    match Raphael::Five.cmp(&Raphael::Two) {
        std::cmp::Ordering::Greater => println!("Raphael::Five is GREATER than Raphael::Two"),
        std::cmp::Ordering::Less => println!("Raphael::Five is LESS than Raphael::Two"),
        std::cmp::Ordering::Equal => println!("Raphael::Five is EQUALS Raphael::Two"),
    }

    println!("\nThe End ...\n");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Daniel {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Raphael {
    One = 5,
    Two = 4,
    Three = 3,
    Four = 2,
    Five = 1,
}

// encounter with compile time error
// This is not possible two enum members have an identical value.

// enum Samuel {
// One = 5,
// Two = 5,
// Three = 4,
// Four = 4,
// Five = 1,
// }
