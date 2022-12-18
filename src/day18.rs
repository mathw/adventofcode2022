use crate::common::day;
use std::{collections::HashSet, str::FromStr};

pub fn run() -> day::Result {
    let input = include_str!("inputs/day18.txt");
    let blob = Blob::from_str(input)?;
    let exposed_faces = blob.count_exposed_faces();
    let exposed_surface_area = blob.external_surface_area();
    Ok((
        Some(format!("{} exposed faces", exposed_faces)),
        Some(format!("Exposed surface area {}", exposed_surface_area)),
    ))
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

    fn exposed_face_cubes(&self) -> impl Iterator<Item = Cube> + '_ {
        self.cubes.iter().flat_map(|c| {
            c.surrounding()
                .into_iter()
                .filter(|sc| !self.cubes.contains(sc))
        })
    }

    fn external_surface_area(&self) -> usize {
        let exposed_cubes: Vec<Cube> = self.exposed_face_cubes().collect();
        exposed_cubes
            .iter()
            .filter(|c| {
                c.surrounding()
                    .into_iter()
                    .any(|sc| !self.cubes.contains(&sc) && !exposed_cubes.contains(&sc))
            })
            .count()
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

#[test]
fn test_part2_full_sample() {
    let blob = Blob::from_str(INPUT).expect("This should parse");
    assert_eq!(blob.external_surface_area(), 58);
}

#[test]
fn test_part2_small_sample() {
    let blob = Blob::from_str(
        "1,1,1
2,1,1",
    )
    .expect("This should parse");
    assert_eq!(blob.external_surface_area(), 10);
}

#[test]
fn test_part2_hollow_sample() {
    let blob = Blob::from_str(
        "1,1,1
3,1,1
2,1,2
2,1,0
2,2,1
2,0,1",
    )
    .expect("This should parse");
    assert_eq!(blob.count_exposed_faces(), 36);
    assert_eq!(blob.external_surface_area(), 30);
}

#[test]
fn test_part2_hollow_sample_2() {
    let blob = Blob::from_str(
        "0,1,1
1,1,1
3,1,1
2,1,2
2,1,0
2,2,1
2,0,1",
    )
    .expect("This should parse");
    assert_eq!(blob.count_exposed_faces(), 40);
    assert_eq!(blob.external_surface_area(), 34);
}

#[test]
fn test_part2_hollow_cube_sample() {
    let blob = Blob::from_str(
        "0,0,0
1,0,0
2,0,0
0,0,1
0,0,2
1,0,1
1,0,2
2,0,1
2,0,2
0,2,0
1,2,0
2,2,0
0,2,1
0,2,2
1,2,1
1,2,2
2,2,1
2,2,2
0,1,0
1,1,0
2,1,0
0,1,1
0,1,2
1,1,2
2,1,1
2,1,2",
    )
    .expect("This should parse");
    assert_eq!(blob.count_exposed_faces(), 60);
    assert_eq!(blob.external_surface_area(), 54);
}

#[test]
fn test_part2_big_hollow_cube_containing_cube() {
    let blob = Blob::from_str(
        "0,0,0
1,0,0
2,0,0
3,0,0
4,0,0
0,0,1
0,0,2
0,0,3
0,0,4
1,0,1
1,0,2
1,0,3
1,0,4
2,0,1
2,0,2
2,0,3
2,0,4,
3,0,1
3,0,2
3,0,3
3,0,4
0,4,0
1,4,0
2,4,0
3,4,0
4,4,0
0,4,1
0,4,2
0,4,3
0,4,4
1,4,1
1,4,2
1,4,3
1,4,4
2,4,1
2,4,2
2,4,3
2,4,4,
3,4,1
3,4,2
3,4,3
3,4,4
0,1,0
1,1,0
2,1,0
3,1,0
4,1,0
0,1,1
0,1,2
0,1,3
0,1,4
1,1,4
2,1,4
3,1,4
4,1,4
4,1,3
4,1,2
4,1,1
0,2,0
1,2,0
2,2,0
3,2,0
4,2,0
0,2,1
0,2,2
0,2,3
0,2,4
1,2,4
2,2,4
3,2,4
4,2,4
4,2,3
4,2,2
4,2,1
0,3,0
1,3,0
2,3,0
3,3,0
4,3,0
0,3,1
0,3,2
0,3,3
0,3,4
1,3,4
2,3,4
3,3,4
4,3,4
4,3,3
4,3,2
4,3,1
2,2,2",
    )
    .expect("This should parse");
    assert_eq!(blob.count_exposed_faces(), 150 + (9 * 6) + 6);
    assert_eq!(blob.external_surface_area(), 150);
}
