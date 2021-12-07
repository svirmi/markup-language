#[derive(Copy, Clone)]
enum State {
    Normal,
    Comment,
    Upper,
    Lower,
} // text transforms

fn machine_cycle(state: State, c: char) -> (Option<char>, State) {
    use self::State::*;
    match state {
        Normal => match c {
            '#' => (None, Comment),
            '^' => (None, Upper),
            '_' => (None, Lower),
            other => (Some(other), Normal),
        },
        Comment => match c {
            '#' => (None, Normal),
            other => (None, Other),
        },
        Upper => match c {},
        Lower => match c {},
    }
}

fn main() {
    println!("Hello, world!");
}
