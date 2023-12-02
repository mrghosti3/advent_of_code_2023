use std::fmt::Debug;
use std::io::BufRead;
use std::u32;

pub fn possible_game_sum<R: BufRead>(input: &mut R) -> u32 {
    let mut sum = 0;
    let mut buf: Vec<u8> = Vec::with_capacity(256);

    while let Ok(read) = input.read_until(b'\n', &mut buf) {
        if read == 0 {
            break;
        }

        sum += match process_line(&buf) {
            Some(game) => {
                if game.is_possible() {
                    game.id
                } else {
                    0
                }
            }
            None => 0,
        };

        buf.clear();
    }

    sum
}

fn process_line(line_buf: &[u8]) -> Option<Game> {
    let mut game = Game::default();
    let mut line_iter = line_buf.splitn(2, |b| *b == b':');
    let game_id: u32 = {
        let game_id_buf = line_iter.next()?.splitn(2, |b| *b == b' ').skip(1).next()?;
        (0..game_id_buf.len())
            .step_by(1)
            .rev()
            .zip(game_id_buf.iter())
            .map(|(i, b)| (*b - 0x30) as u32 * 10u32.pow(i as u32))
            .sum()
    };

    game.id = game_id;

    let set_iter = line_iter
        .next()?
        .split(|b| *b == b';')
        .enumerate()
        .map(|(set_i, set)| {
            (
                set_i,
                set.splitn(3, |b| *b == b',')
                    .map(|cube| cube[1..].splitn(2, |b| *b == b' ').collect::<Vec<&[u8]>>()),
            )
        });

    for (set_i, set) in set_iter {
        game.sets.push(GameSet::default());
        let gset_ref = &mut game[set_i];

        for cube in set {
            let (cube_count, cube_t) = (cube[0], cube[1]);
            let cube_count = (0..cube_count.len())
                .step_by(1)
                .rev()
                .zip(cube_count.iter())
                .map(|(i, b)| (*b - 0x30) as u32 * 10u32.pow(i as u32))
                .sum();

            let cube_t: Cube = if cube_t[cube_t.len() - 1] == b'\n' {
                cube_t[..cube_t.len() - 1].try_into().unwrap()
            } else {
                cube[1].try_into().unwrap()
            };

            gset_ref[cube_t] = cube_count;
        }
    }

    Some(game)
}

const CUBE_SET: GameSet = GameSet::new(12, 13, 14);

#[derive(Debug, Default)]
pub struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

impl Game {
    #[inline]
    pub fn is_possible(&self) -> bool {
        for set in self.sets.iter() {
            if !set.is_possible() {
                return false;
            }
        }

        true
    }
}

impl std::ops::Index<usize> for Game {
    type Output = GameSet;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sets[index]
    }
}

impl std::ops::IndexMut<usize> for Game {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.sets[index]
    }
}

#[derive(Debug)]
#[repr(u8)]
enum Cube {
    Red,
    Green,
    Blue,
}

impl TryFrom<&[u8]> for Cube {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"red" => Ok(Self::Red),
            b"green" => Ok(Self::Green),
            b"blue" => Ok(Self::Blue),
            _ => Err("unknown cube type"),
        }
    }
}

impl From<usize> for Cube {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Red,
            1 => Self::Green,
            2 => Self::Blue,
            _ => Self::Blue,
        }
    }
}

#[derive(Default)]
pub struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameSet {
    #[inline]
    pub const fn new(red: u32, green: u32, blue: u32) -> Self {
        GameSet { red, green, blue }
    }

    #[inline]
    pub fn is_possible(&self) -> bool {
        return self.red <= CUBE_SET.red
            && self.green <= CUBE_SET.green
            && self.blue <= CUBE_SET.blue;
    }
}

impl std::ops::Index<Cube> for GameSet {
    type Output = u32;

    fn index(&self, index: Cube) -> &Self::Output {
        match index {
            Cube::Red => &self.red,
            Cube::Green => &self.green,
            Cube::Blue => &self.blue,
        }
    }
}

impl std::ops::IndexMut<Cube> for GameSet {
    fn index_mut(&mut self, index: Cube) -> &mut Self::Output {
        match index {
            Cube::Red => &mut self.red,
            Cube::Green => &mut self.green,
            Cube::Blue => &mut self.blue,
        }
    }
}

impl Debug for GameSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GameSet {{ r: {:04}, g: {:04}, b: {:04} }}",
            self.red, self.green, self.blue
        )
    }
}
