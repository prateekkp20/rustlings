// enums3.rs
// Address all the TODOs to make the tests pass!



enum Message {
    Move (Point),
    Echo (String),
    ChangeColor(i32,i32,i32),
    Quit,
    // TODO: implement the message variant types based on their usage below
}

struct Point {
    x: i32,
    y: i32,
}

struct State {
    color: (i32, i32, i32),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (i32, i32, i32)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::Move (Point{x,y}) => self.move_position(Point{ x ,y }),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(q,r,t) => self.change_color( (q,r,t)),
            Message::Quit => self.quit(),
        }
        // TODO: create a match expression to process the different message variants
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}
