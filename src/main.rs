fn main() {
    println!("Hello, world!");
}

enum Frame {
    Strike,
    Spare,
    Completed(u32, u32),
    Incomplete(u32)
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
              self.frames.push(Frame::Completed(first, pins))

            }
            _ => self.frames.push(Frame::Incomplete(pins))
        };

        Game {
            frames: self.frames,
        }
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
        assert_eq!(Game::new().roll(10).roll(9).score(), 29)
    }

    #[test]
    #[ignore]
    fn perfect_game() {}
}

