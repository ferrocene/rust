//@ edition: ..2021
//@ compile-flags: --test
//@ run-flags: --test-threads=1 --nocapture
//@ run-fail
//@ check-run-results
//@ exec-env:RUST_BACKTRACE=0
//@ normalize-stdout: "finished in \d+\.\d+s" -> "finished in $$TIME"
//@ normalize-stdout: "TypeId\(0x[0-9a-f]+\)" -> "TypeId($$HEX)"
//@ needs-threads
//@ needs-unwind (panic)

#[test]
#[should_panic = "message"]
#[expect(non_fmt_panics)]
fn should_panic_with_substring_panics_with_non_string_value() {
    panic!(123);
}
