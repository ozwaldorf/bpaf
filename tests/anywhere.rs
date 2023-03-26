use bpaf::*;

#[test]
fn parse_anywhere_no_catch() {
    let a = short('a').req_flag(());
    let b = positional::<usize>("x");
    let ab = construct!(a, b).anywhere();
    let c = short('c').switch();
    let parser = construct!(ab, c).to_options();

    let r = parser
        .run_inner(Args::from(&["-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        r,
        "Expected <x>, got \"-a\". Pass --help for usage information"
    );

    let r = parser
        .run_inner(Args::from(&["-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Couldn't parse \"221b\": invalid digit found in string");

    let r = parser
        .run_inner(Args::from(&["-c", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        r,
        "Expected <x>, got \"-c\". Pass --help for usage information"
    );

    let r = parser
        .run_inner(Args::from(&["-c", "-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Couldn't parse \"221b\": invalid digit found in string");

    let r = parser
        .run_inner(Args::from(&["-a", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Expected an argument <x>, got -c");

    let r = parser
        .run_inner(Args::from(&["-a", "221b", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Couldn't parse \"221b\": invalid digit found in string");

    let r = parser
        .run_inner(Args::from(&["3", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        r,
        "Expected <x>, got \"3\". Pass --help for usage information"
    );
}

#[test]
fn parse_anywhere_catch_required() {
    let a = short('a').req_flag(());
    let b = positional::<usize>("x");
    let ab = construct!(a, b).anywhere().catch();
    let c = short('c').switch();
    let parser = construct!(ab, c).to_options();

    let r = parser
        .run_inner(Args::from(&["-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        r,
        "Expected <x>, got \"-a\". Pass --help for usage information"
    );

    let r = parser
        .run_inner(Args::from(&["-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        r,
        "Expected -a x, got \"-a\". Pass --help for usage information"
    );

    let r = parser
        .run_inner(Args::from(&["-c", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        r,
        "Expected <x>, got \"-c\". Pass --help for usage information"
    );

    let r = parser
        .run_inner(Args::from(&["-c", "-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        r,
        "Expected -a x, got \"-c\". Pass --help for usage information"
    );

    let r = parser
        .run_inner(Args::from(&["-a", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        r,
        "Expected -a x, got \"-a\". Pass --help for usage information"
    );

    let r = parser
        .run_inner(Args::from(&["-a", "221b", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        r,
        "Expected -a x, got \"-a\". Pass --help for usage information"
    );

    let r = parser
        .run_inner(Args::from(&["3", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(
        r,
        "Expected <x>, got \"3\". Pass --help for usage information"
    );
}

#[test]
fn parse_anywhere_catch_optional() {
    let a = short('a').req_flag(());
    let b = positional::<usize>("x");
    let ab = construct!(a, b).anywhere().catch().optional();
    let c = short('c').switch();
    let parser = construct!(ab, c).to_options();

    let r = parser
        .run_inner(Args::from(&["-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["-c", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["-c", "-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["-a", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["-a", "221b", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["3", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "3 is not expected in this context");
}