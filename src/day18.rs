use crate::common::day;
use std::{collections::HashSet, str::FromStr};

pub fn run() -> day::Result {
    let input = include_str!("inputs/day18.txt");
    let blob = Blob::from_str(input)?;
    let exposed_faces = blob.count_exposed_faces();
    Ok((Some(format!("{} exposed faces", exposed_faces)), None))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn surrounding(&self) -> [Cube; 6] {
        [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .map(|v| *self + v)
    }
}

impl std::ops::Add<(i32, i32, i32)> for Cube {
    type Output = Cube;

    fn add(self, (x, y, z): (i32, i32, i32)) -> Self::Output {
        Cube {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }
}

struct Blob {
    cubes: HashSet<Cube>,
}

impl Blob {
    fn from_cubes(i: impl Iterator<Item = Cube>) -> Self {
        Self { cubes: i.collect() }
    }

    fn count_exposed_faces(&self) -> usize {
        let mut exposed_faces = self.cubes.len() * 6;

        for cube in self.cubes.iter() {
            // discount every surrounding cube that is in the blob
            // this should work with overlapping neighbours as we already counted overlapping labels
            exposed_faces -= cube
                .surrounding()
                .into_iter()
                .filter(|s| self.cubes.contains(s))
                .count()
        }

        exposed_faces
    }
}

impl FromStr for Blob {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Blob::from_cubes(
            s.lines()
                .map(|line| {
                    let mut parts = line.trim().split(',');
                    if let (Some(x), Some(y), Some(z)) = (parts.next(), parts.next(), parts.next())
                    {
                        Ok::<Cube, Box<dyn std::error::Error>>(Cube::new(
                            x.parse()?,
                            y.parse()?,
                            z.parse()?,
                        ))
                    } else {
                        Err(format!("Input line '{}' didn't have three parts", line).into())
                    }
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter(),
        ))
    }
}

#[test]
fn test_part1_small_sample() {
    let blob = Blob::from_str(
        "1,1,1
2,1,1",
    )
    .expect("This should parse");
    assert_eq!(blob.count_exposed_faces(), 10);
}

#[cfg(test)]
const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

#[test]
fn test_part1_full_sample() {
    let blob = Blob::from_str(INPUT).expect("This should parse");
    assert_eq!(blob.count_exposed_faces(), 64);
}
