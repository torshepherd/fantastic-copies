struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

fn example() {
    let mut overwritten = PrintOnDrop("drops when overwritten");
    overwritten = PrintOnDrop("drops when scope ends");

    let tuple = (PrintOnDrop("Tuple first"), PrintOnDrop("Tuple second"));

    let moved;
    // No destructor run on assignment.
    moved = PrintOnDrop("Drops when moved");
    // Drops now, but is then uninitialized.
    moved;

    // Uninitialized does not drop.
    let uninitialized: PrintOnDrop;

    // After a partial move, only the remaining fields are dropped.
    let mut partial_move = (PrintOnDrop("first"), PrintOnDrop("forgotten"));
    // Perform a partial move, leaving only `partial_move.0` initialized.
    core::mem::forget(partial_move.1);
    // When partial_move's scope ends, only the first field is dropped.
}

fn foo() {
    let v: Vec<i32> = vec![1, 2, 3];
    let other = v;

    let mut first = PrintOnDrop("first");
    let second = first;
}

fn main() {
    foo();
}
