#[derive(Debug, Clone)]
struct Space {
    cells: Vec<bool>,
    size: usize,
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Space {
    fn index(&self, Coord { x, y, z, w }: Coord) -> Option<usize> {
        if x >= 0
            && (x as usize) < self.size
            && y >= 0
            && (y as usize) < self.size
            && z >= 0
            && (z as usize) < self.size
            && w >= 0
            && (w as usize) < self.size
        {
            Some(
                (w as usize * self.size * self.size * self.size)
                    + (z as usize * self.size * self.size)
                    + (y as usize * self.size)
                    + x as usize,
            )
        } else {
            None
        }
    }

    fn get(&self, coord: Coord) -> Option<bool> {
        self.index(coord).map(|i| self.cells[i])
    }

    fn get_mut(&mut self, coord: Coord) -> Option<&mut bool> {
        self.index(coord).map(move |i| &mut self.cells[i])
    }

    fn num_living_neighbours(&self, coord: Coord) -> usize {
        (-1..=1)
            .flat_map(|w| {
                (-1..=1).flat_map(move |z| {
                    (-1..=1).flat_map(move |y| {
                        (-1..=1).map(move |x| {
                            if x == 0 && y == 0 && z == 0 && w == 0 {
                                0
                            } else {
                                self.get(Coord {
                                    x: coord.x + x,
                                    y: coord.y + y,
                                    z: coord.z + z,
                                    w: coord.w + w,
                                })
                                .unwrap_or(false) as usize
                            }
                        })
                    })
                })
            })
            .sum()
    }

    fn new(size: usize) -> Self {
        let cells = vec![false; size * size * size * size];
        Self { cells, size }
    }

    fn parse_stdin(num_cycles: usize) -> Self {
        use std::io::BufRead;
        let rows = std::io::stdin()
            .lock()
            .lines()
            .map(|l| {
                let line = l.unwrap();
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let size = rows[0].len().max(rows.len()) + (4 * num_cycles);
        let mut s = Self::new(size);
        let offset = (size - num_cycles) / 2;
        let mid = size / 2;
        for (y, row) in rows.into_iter().enumerate() {
            for (x, cell) in row.into_iter().enumerate() {
                let coord = Coord {
                    x: (x + offset) as i32,
                    y: (y + offset) as i32,
                    z: mid as i32,
                    w: mid as i32,
                };
                *s.get_mut(coord).unwrap() = cell;
            }
        }
        s
    }

    fn print_slice(&self, z: i32, w: i32) {
        for y in 0..(self.size as i32) {
            for x in 0..(self.size as i32) {
                let coord = Coord { x, y, z, w };
                if self.get(coord).unwrap() {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    fn tick(&mut self) {
        let other = self.clone();
        for w in 0..(self.size as i32) {
            for z in 0..(self.size as i32) {
                for y in 0..(self.size as i32) {
                    for x in 0..(self.size as i32) {
                        let coord = Coord { x, y, z, w };
                        let num_living_neighbours = other.num_living_neighbours(coord);
                        if other.get(coord).unwrap() {
                            if num_living_neighbours != 2 && num_living_neighbours != 3 {
                                *self.get_mut(coord).unwrap() = false;
                            }
                        } else {
                            if num_living_neighbours == 3 {
                                *self.get_mut(coord).unwrap() = true;
                            }
                        }
                    }
                }
            }
        }
    }

    fn num_active(&self) -> usize {
        self.cells.iter().filter(|&&c| c).count()
    }
}

fn main() {
    let mut space = Space::parse_stdin(3);
    for _ in 0..6 {
        space.tick();
        space.print_slice(space.size as i32 / 2, space.size as i32 / 2);
        println!("===");
    }
    println!("{}", space.num_active());
}
