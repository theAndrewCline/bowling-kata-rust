fn main() {
    println!("Hello, world!");

    let game = Game::new().roll(10).roll(5).roll(4);

    println!("{:?}", game.frames);
}

#[derive(Debug)]
enum Frame {
    Strike,
    Spare,
    Completed(u32, u32),
    Incomplete(u32),
}

struct Game {
    frames: Vec<Frame>,
}

impl Game {
    fn new() -> Game {
        Game { frames: vec![] }
    }

    fn roll(mut self, pins: u32) -> Game {
        let cur_frame = self.frames.pop();

        match cur_frame {
            Some(Frame::Incomplete(first)) => {
                if first + pins == 10 {
                    self.frames.push(Frame::Spare)
                } else {
                    self.frames.push(Frame::Completed(first, pins))
                }
            }

            Some(Frame::Completed(_, _)) | Some(Frame::Spare) | Some(Frame::Strike) => {
                self.frames.push(cur_frame.unwrap());
                if pins >= 10 {
                    self.frames.push(Frame::Strike)
                } else {
                    self.frames.push(Frame::Incomplete(pins))
                }
            }

            None => {
                if pins >= 10 {
                    self.frames.push(Frame::Strike)
                } else {
                    self.frames.push(Frame::Incomplete(pins))
                }
            }
        }

        return Game {
            frames: self.frames,
        };
    }

    fn score(self) -> u32 {
        self.frames.into_iter().fold(0, |acc, value| match value {
            Frame::Completed(first, second) => acc + first + second,
            Frame::Incomplete(score) => acc + score,
            Frame::Spare => acc + 10,
            Frame::Strike => acc + 10,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_game() {
        assert_eq!(Game::new().score(), 0)
    }

    #[test]
    fn roll_0() {
        assert_eq!(Game::new().roll(0).score(), 0)
    }

    #[test]
    fn roll_7() {
        assert_eq!(Game::new().roll(7).score(), 7)
    }

    #[test]
    fn roll_strike() {
        assert_eq!(Game::new().roll(10).score(), 10)
    }

    #[test]
    fn roll_after_strike() {
        let game = Game::new().roll(10).roll(4).roll(5);

        println!("{:?}", game.frames);

        assert_eq!(game.score(), 19)
    }

    #[test]
    #[ignore]
    fn perfect_game() {}
}
