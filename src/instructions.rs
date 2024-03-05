#[derive(Clone, Debug)]
pub enum Instruction {
    Goto(Coord, Coord),
    Face(Direction),
    Move(f32),
    Pen(bool),
    Clear,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Coord {
    Center,
    Absolute(f32),
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    West,
    East,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
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
        macro_rules! assert_end_params {
            () => {
                if args.next().is_some() {
                    return Err(format!("Too many parameters!"));
                }
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
                assert_end_params!();
                Ok(Self::Goto(x, y))
            }

            "face" => {
                let dir = match args.next() {
                    Some(dir) => Direction::try_from(dir)?,
                    None => return missing_pararm!(direction),
                };
                assert_end_params!();
                Ok(Self::Face(dir))
            }

            "move" => {
                let amount = match args.next() {
                    Some(amount) => amount
                        .parse()
                        .map_err(|_| format!("Invalid number value"))?,
                    None => return missing_pararm!(direction),
                };
                assert_end_params!();
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
                assert_end_params!();
                Ok(Self::Pen(state))
            }

            "clear" => {
                assert_end_params!();
                Ok(Self::Clear)
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
            "north" => Self::North,
            "south" => Self::South,
            "west" => Self::West,
            "east" => Self::East,
            "northwest" => Self::NorthWest,
            "southwest" => Self::SouthWest,
            "northeast" => Self::NorthEast,
            "southeast" => Self::SouthEast,
            _ => return Err(format!("Invalid direction")),
        })
    }
}

pub fn parse_file(file: &str) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();
    for line in file.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        let instr = Instruction::try_from(line)?;
        instructions.push(instr)
    }
    Ok(instructions)
}
