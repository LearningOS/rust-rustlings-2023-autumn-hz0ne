// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let mut x = 100;
    let z = &mut x;
    let y = &mut x;
    *z += 1000;
    *y += 100;
    
    assert_eq!(x, 1200);
}
