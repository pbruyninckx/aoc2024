use anyhow::Error;
use num_traits::cast::AsPrimitive;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, IndexMut, Mul, Rem, RemAssign};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i64> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Rem for Pos {
    type Output = Pos;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.rem_euclid(rhs.x),
            y: self.y.rem_euclid(rhs.y),
        }
    }
}

impl RemAssign for Pos {
    fn rem_assign(&mut self, rhs: Self) {
        self.x = self.x.rem_euclid(rhs.x);
        self.y = self.y.rem_euclid(rhs.y);
    }
}

pub struct Map<T> {
    pub size: Pos,
    pub data: Vec<Vec<T>>,
}

pub trait ConvertibleFromChar {
    fn from_char(c: char) -> Result<Self, Error>
    where
        Self: Sized;
}

impl ConvertibleFromChar for char {
    fn from_char(c: char) -> Result<char, Error> {
        Ok(c)
    }
}

impl ConvertibleFromChar for u8 {
    fn from_char(c: char) -> Result<u8, Error> {
        {
            c.to_digit(10)
                .map(|d| d.as_())
                .ok_or(Error::msg("non-digit found in input"))
        }
    }
}

impl<T: 'static + ConvertibleFromChar + Copy> Map<T> {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let data: Vec<Vec<T>> = s
            .lines()
            .map(|l| l.chars().map(T::from_char).collect())
            .collect::<Result<Vec<_>, _>>()?;
        let size = Pos {
            x: data.first().ok_or(Error::msg("Empty data"))?.len() as i64,
            y: data.len() as i64,
        };
        Ok(Self { data, size })
    }
}

impl<T> Map<T> {
    pub fn contains(&self, pos: &Pos) -> bool {
        0 <= pos.x && pos.x < self.size.y && 0 <= pos.y && pos.y < self.size.y
    }

    pub fn neighbors(&self, pos: &Pos) -> Vec<Pos> {
        [-1, 1]
            .iter()
            .flat_map(|i| {
                [
                    Pos {
                        x: pos.x + i,
                        y: pos.y,
                    },
                    Pos {
                        x: pos.x,
                        y: pos.y + i,
                    },
                ]
            })
            .filter(|pos| self.contains(pos))
            .collect()
    }
}

impl<T> Index<&Pos> for Map<T> {
    type Output = T;

    fn index(&self, pos: &Pos) -> &Self::Output {
        &self.data[pos.y as usize][pos.x as usize]
    }
}

impl<T> IndexMut<&Pos> for Map<T> {
    fn index_mut(&mut self, pos: &Pos) -> &mut Self::Output {
        &mut self.data[pos.y as usize][pos.x as usize]
    }
}

impl<T: std::fmt::Debug> Display for Map<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.data.iter() {
            writeln!(f, "{:?}", line)?;
        }
        Ok(())
    }
}
