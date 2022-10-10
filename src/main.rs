fn main() {
    println!("Hello, world!");

    let game = Game::new().roll(10).roll(5).roll(4);

    println!("{:?}", game.frames);
}

#[derive(Debug, Clone, Copy)]
enum Frame {
    Strike,
    Spare,
    Completed(u32, u32),
    Incomplete(u32),
}

#[derive(Debug, Clone)]
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

    fn score(&self) -> u32 {
        let mut score: u32 = 0;

        for (index, frame) in self.frames.clone().into_iter().enumerate() {
            let previous_frame = if index > 0 {
                self.frames.get(index - 1)
            } else {
                None
            };

            let second_previous_frame = if index > 1 {
                self.frames.get(index - 2)
            } else {
                None
            };

            match frame {
                Frame::Incomplete(first) => score += first,

                Frame::Completed(first, second) => {
                    match previous_frame {
                        Some(f) => match f {
                            Frame::Strike | Frame::Spare => score += 10,
                            _ => {}
                        },
                        None => {}
                    };

                    match second_previous_frame {
                        Some(f) => match f {
                            Frame::Strike => score += 10,
                            _ => {}
                        },
                        None => {}
                    };

                    score += first + second
                }

                Frame::Spare => {
                    match previous_frame {
                        Some(f) => match f {
                            Frame::Strike | Frame::Spare => score += 10,
                            _ => {}
                        },
                        None => {}
                    };

                    match second_previous_frame {
                        Some(f) => match f {
                            Frame::Strike => score += 10,
                            _ => {}
                        },
                        None => {}
                    };

                    score += 10
                }

                Frame::Strike => {
                    match previous_frame {
                        Some(f) => match f {
                            Frame::Strike | Frame::Spare => score += 10,
                            _ => {}
                        },
                        None => {}
                    };

                    match second_previous_frame {
                        Some(f) => match f {
                            Frame::Strike => score += 10,
                            _ => {}
                        },
                        None => {}
                    };

                    score += 10
                }
            }
        }

        return score;
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

        assert_eq!(game.score(), 29)
    }

    #[test]
    fn perfect_game() {
        let game = Game::new()
            .roll(10)
            .roll(10)
            .roll(10)
            .roll(10)
            .roll(10)
            .roll(10)
            .roll(10)
            .roll(10)
            .roll(10)
            .roll(10)
            .roll(10);

        assert_eq!(game.score(), 300)
    }
}
