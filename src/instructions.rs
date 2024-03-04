#[derive(Debug)]
pub enum Instruction {
    Goto(Coord, Coord),
    Face(Direction),
    Move(f32),
    Pen(bool),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Coord {
    Center,
    Absolute(f32),
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut args = line.split_whitespace();
        let Some(operation) = args.next() else {
            return Err(format!("Missing operation"));
        };

        macro_rules! missing_pararm {
            ( $name:ident ) => {
                Err(format!(
                    "Expected argument for parameter `{}`",
                    stringify!($name)
                ))
            };
        }

        match operation.to_lowercase().as_str() {
            "goto" => {
                let x = match args.next() {
                    Some(x) => Coord::try_from(x)?,
                    None => return missing_pararm!(x),
                };
                let y = match args.next() {
                    Some(y) => Coord::try_from(y)?,
                    None if x == Coord::Center => x,
                    None => return missing_pararm!(y),
                };
                Ok(Self::Goto(x, y))
            }

            "face" => {
                let dir = match args.next() {
                    Some(dir) => Direction::try_from(dir)?,
                    None => return missing_pararm!(direction),
                };
                Ok(Self::Face(dir))
            }

            "move" => {
                let amount = match args.next() {
                    Some(amount) => amount
                        .parse()
                        .map_err(|_| format!("Invalid number value"))?,
                    None => return missing_pararm!(direction),
                };
                Ok(Self::Move(amount))
            }

            "pen" => {
                let Some(state) = args.next() else {
                    return missing_pararm!(state);
                };
                let state = match state.to_lowercase().as_str() {
                    "down" => true,
                    "up" => false,
                    _ => return Err(format!("Invalid pen state")),
                };
                Ok(Self::Pen(state))
            }

            _ => Err(format!("Unknown operation `{}`", operation)),
        }
    }
}

impl TryFrom<&str> for Coord {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.to_lowercase() == "center" {
            return Ok(Self::Center);
        }
        let Ok(number) = value.parse::<f32>() else {
            return Err(format!("Invalid coordinate value"));
        };
        Ok(Self::Absolute(number))
    }
}

impl TryFrom<&str> for Direction {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value.to_lowercase().as_str() {
            "left" => Self::Left,
            "right" => Self::Right,
            "up" => Self::Up,
            "down" => Self::Down,
            _ => return Err(format!("Invalid direction")),
        })
    }
}

pub fn parse_file(file: &str) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();
    for line in file.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let instr = Instruction::try_from(line)?;
        instructions.push(instr)
    }
    Ok(instructions)
}
