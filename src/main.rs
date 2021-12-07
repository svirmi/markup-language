#[derive(Copy, Clone)]
enum State {
    Normal,
    Comment,
    Upper,
    Lower,
} // text transforms

fn machine_cycle(state: State, c: char) -> (Option<char>, MachineState) {
    use self::State::*;
    match state {
        Normal => {}
        Comment => {}
        Upper => {}
        Lower => {}
    }
}

fn main() {
    println!("Hello, world!");
}
