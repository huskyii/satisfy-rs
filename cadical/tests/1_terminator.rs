use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use cadical::{self, TimeoutTerminator};

#[allow(dead_code)]
fn bench<T>(func: impl FnOnce() -> T) -> (Duration, T) {
    let t_start = Instant::now();
    let res = func();
    let t_end = Instant::now();
    (t_end - t_start, res)
}

#[test]
fn test_terminator() {
    let mut solver = cadical::new().unwrap().finish();

    let mut pb = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pb.push("tests");
    pb.push("prime65537.cnf");

    let f = File::open(pb.as_path()).unwrap();
    let mut buffer = String::new();
    let mut reader = BufReader::new(f);
    // skip first line
    for _ in 0..1 {
        reader.read_line(&mut buffer).unwrap();
    }
    buffer.clear();
    reader.read_to_string(&mut buffer).unwrap();
    for lit_s in buffer.split_ascii_whitespace() {
        solver = solver.add_lit(lit_s.parse().unwrap());
    }

    // measure how much time it takes to solving this formula
    // let (elapsed, res) = bench(|| solver.solve());
    // eprintln!("elapsed: {:?}\n{:?}", elapsed, res);
    // return;

    // then adjust timeout here to make sure it terminated before solved
    let t = TimeoutTerminator::new(Duration::from_millis(10));

    solver.set_terminator(Some(t));

    let mut solver = match solver.solve() {
        cadical::Result::Unknown(s) => {
            // terminated before it got solved, thus unknown
            s
        }
        _ => {
            assert!(false);
            unreachable!();
        }
    };

    // remove Terminator, this will drop previous terminator
    solver.set_terminator(cadical::NoneTerminator);

    let _solver = match solver.solve() {
        cadical::Result::Unsat(s) => s,
        _ => {
            assert!(false);
            unreachable!();
        }
    };
}
