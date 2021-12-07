#[derive(Copy, Clone)]
enum State {
    Normal,
    Comment,
    Upper,
    Lower,
} // text transforms

fn machine_cycle(state: State, c: char) -> (Option<char>, State) {
    use self::State::*;
    match (state, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),
    }
}

fn main() {
    println!("Hello, world!");
}
