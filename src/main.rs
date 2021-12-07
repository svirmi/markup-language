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
        (Comment, '#') => (None, Normal),
        (Comment, other) => (None, Comment),
        (Upper, '^') => (None, Normal),
        (Upper, other) => (Some(___), Upper),
        (Lower, '_') => (None, Normal),
        (Lower, other) => (Some(___), Lower),
    }
}

fn main() {
    println!("Hello, world!");
}
