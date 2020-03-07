use cadical::{self, Clause, Lit};

#[test]
fn test_basic() {
    // classic dress code example
    let tie = Lit::new(1).unwrap();
    let shirt = Lit::new(2).unwrap();

    // wear shirt without tie
    let mut c0 = Clause::new();
    c0.push(!tie);
    c0.push(shirt);
    // wear shirt and tie
    let mut c1 = Clause::new();
    c1.push(tie);
    c1.push(shirt);
    // only one of shirt and tie
    let mut c2 = Clause::new();
    c2.push(!tie);
    c2.push(!shirt);

    let solver = cadical::new().unwrap();
    let res = solver.solve();
    let solver = if let cadical::Result::Sat(s) = res {
        assert!(true);
        s
    } else {
        assert!(false);
        unreachable!();
    };

    let res = solver.add_clause(c0.iter()).add_clause(c1.iter()).add_clause(c2.iter()).solve();
    let solver = if let cadical::Result::Sat(s) = res {
        assert_eq!(s.val(tie), false);
        assert_eq!(s.val(shirt), true);
        s
    } else {
        assert!(false);
        unreachable!();
    };

    // incremental solving
    // force wear tie
    let res = solver.assume(tie).solve();
    let solver = if let cadical::Result::Unsat(s) = res {
        // tie is responsible for unsat
        assert_eq!(s.failed(tie), true);
        // and shirt is not
        assert_eq!(s.failed(shirt), false);
        s
    } else {
        assert!(false);
        unreachable!();
    };

    // force not wear shirt
    let res = solver.assume(!shirt).solve();
    let _solver = if let cadical::Result::Unsat(s) = res {
        // !shirt is responsible for unsat
        assert_eq!(s.failed(!shirt), true);
        // and tie is not
        assert_eq!(s.failed(tie), false);
        // and shirt is not
        assert_eq!(s.failed(shirt), false);
        s
    } else {
        assert!(false);
        unreachable!();
    };

    // drop _solver and release underlying C++ object
}
