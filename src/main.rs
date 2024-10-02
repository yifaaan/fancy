use terminal_size::terminal_size;

mod terminal_size;
fn main() {
    println!("{:?}", terminal_size());
}
