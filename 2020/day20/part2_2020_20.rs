// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 20 Part 2

// moved `impl Tile { ... } to the bottom of the file for part 2, to keep focus on new stuff.
// It's unchanged from part 1.
#[derive(PartialEq, Clone, Copy)]
struct Tile {
    id: u16,
    pixels: [[bool; 10]; 10],
}

struct Picture {
    pixels: Vec<bool>,
    size: usize,
}

impl Picture {
    /// utility function to convert an (x, y) coordinate pair into an index into the pixel array
    #[inline]
    const fn coords_to_index(&self, x: usize, y: usize) -> usize {
        (y * self.size) + x
    }

    /// flips the picture along the horizontal
    fn flip(&mut self) {
        self.pixels
            .chunks_mut(self.size)
            .for_each(|chunk| chunk.reverse());
    }

    /// rotates the picture 90 degrees clockwise
    fn rotate(&mut self) {
        for y in 0..self.size {
            for x in y..self.size {
                let xy = self.coords_to_index(x, y);
                let yx = self.coords_to_index(y, x);
                self.pixels.swap(xy, yx);
            }
        }
        self.flip();
    }
    const MONSTER_COORD_OFFSETS: [(usize, usize); 15] = {
        use std::mem::{transmute, MaybeUninit};
        let pattern = [
            b"                  # ",
            b"#    ##    ##    ###",
            b" #  #  #  #  #  #   ",
        ];

        let mut i = 0;
        let mut arr: [MaybeUninit<(usize, usize)>; 15] = [MaybeUninit::uninit(); 15];
        let mut y = 0;
        while y < 3 {
            let mut x = 0;
            while x < 20 {
                if pattern[y][x] == b'#' {
                    arr[i] = MaybeUninit::new((x, y));
                    i += 1;
                }
                x += 1;
            }
            y += 1;
        }
        assert!(i == 15, "unexpected number of elements");

        // SAFETY: the above assert ensures that all elements are initialized
        unsafe { transmute::<[MaybeUninit<(usize, usize)>; 15], [(usize, usize); 15]>(arr) }
    };

    /// checks if a monster is contained within the 3x20 bounding box with the top-left-most corner
    /// at (x, y).
    fn contains_monster_at(&self, x: usize, y: usize) -> bool {
        Self::MONSTER_COORD_OFFSETS
            .into_iter()
            .all(|(off_x, off_y)| self[(x + off_x, y + off_y)])
    }

    fn has_visible_monster(&self) -> bool {
        let max_check_y = self.size.saturating_sub(3);
        let max_check_x = self.size.saturating_sub(20);

        for y in 0..max_check_y {
            for x in 0..max_check_x {
                if self.contains_monster_at(x, y) {
                    return true;
                }
            }
        }
        false
    }

    // start with the logic from part 1 more-or-less unchanged
    fn collect_from(iter: impl Iterator<Item = Tile>) -> Self {
        use std::collections::{HashMap, VecDeque};
        let mut queue: VecDeque<Tile> = iter.collect();
        let mut grid: HashMap<(i8, i8), Tile> = HashMap::with_capacity(queue.len());
        grid.insert((0, 0), queue.pop_front().unwrap());
        'outer: while let Some(mut tile) = queue.pop_front() {
            let mut pos: Option<(i8, i8)> = None;
            macro_rules! locator_loop {
                {} => {
                    'locator: for _ in 0..4 {
                        tile.rotate();
                        for ((x, y), neighbor) in grid.iter() {
                            macro_rules! check_match {
                                ($self_edge: ident, $other_edge: ident, $pos_expr: expr) => {
                                    if tile.$self_edge() == neighbor.$other_edge() {
                                        pos = Some($pos_expr);
                                        break 'locator;
                                    }
                                };
                            }
                            check_match!(top_edge, bottom_edge, (*x, y + 1));
                            check_match!(bottom_edge, top_edge, (*x, y - 1));
                            check_match!(right_edge, left_edge, (x - 1, *y));
                            check_match!(left_edge, right_edge, (x + 1, *y));
                        }
                    }
                };
            }
            locator_loop! {}
            if pos.is_none() {
                tile.flip();
                locator_loop! {}
            }

            let Some((x, y)) = pos else {
                queue.push_back(tile);
                continue 'outer;
            };

            assert!(!grid.contains_key(&(x, y)), "duplicate positions");

            macro_rules! check_pos {
                (n:($x: expr, $y: expr), tile.$tile_edge: ident(), n.$other_edge: ident()) => {
                    assert!(grid
                        .get(&($x, $y))
                        .is_none_or(|t| t.$other_edge() == tile.$tile_edge()));
                };
            }

            check_pos!(n: (x - 1, y), tile.left_edge(), n.right_edge());
            check_pos!(n: (x + 1, y), tile.right_edge(), n.left_edge());
            check_pos!(n: (x, y - 1), tile.top_edge(), n.bottom_edge());
            check_pos!(n: (x, y + 1), tile.bottom_edge(), n.top_edge());
            grid.insert((x, y), tile);
        }
        let mut min_x = i8::MAX;
        let mut min_y = i8::MAX;
        let mut max_x = i8::MIN;
        let mut max_y = i8::MIN;

        for &(x, y) in grid.keys() {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                assert!(grid.contains_key(&(x, y)), "discontinuity at ({x}, {y})");
            }
        }

        let width = usize::from(min_x.abs_diff(max_x) + 1) * 8;
        let height = usize::from(min_y.abs_diff(max_y) + 1) * 8;
        assert_eq!(width, height, "picture is not square, that's not fair");
        let size = width;

        let mut pixels: Vec<bool> = Vec::with_capacity(size * size);
        // for each y level, go through each sublevel 1..=8, and append each tile at the y level's
        // row to `pixels`.
        //
        // Skip the edges at sub_y=0 and sub_y=9, as they're "not part of the actual image"
        // according to the problem description.
        //
        // Goes through each row 8 times, which is not optimal but is fast enough (TM).
        for y in min_y..=max_y {
            for sub_y in 1..=8 {
                for x in min_x..=max_x {
                    // skip sub_x=0 and sub_x=9 for the same reason as above
                    pixels.extend_from_slice(&grid[&(x, y)].pixels[sub_y][1..=8]);
                }
            }
        }

        debug_assert_eq!(pixels.len(), size * size);

        let mut ret = Self { pixels, size };

        macro_rules! monster_check {
            () => {
                if ret.has_visible_monster() {
                    return ret;
                }
            };
        }

        monster_check!();
        for _ in 0..3 {
            ret.rotate();
            monster_check!();
        }
        ret.flip();
        monster_check!();
        for _ in 0..3 {
            ret.rotate();
            monster_check!();
        }
        ret
    }

    fn into_count(mut self) -> usize {
        let mut monster_locations: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.size.saturating_sub(3) {
            for x in 0..self.size.saturating_sub(20) {
                if self.contains_monster_at(x, y) {
                    monster_locations.push((x, y));
                }
            }
        }

        for (x, y) in monster_locations {
            Self::MONSTER_COORD_OFFSETS
                .into_iter()
                .for_each(|(off_x, off_y)| self[(x + off_x, y + off_y)] = false);
        }
        self.pixels.into_iter().filter(|pix| *pix).count()
    }
}

impl std::ops::Index<(usize, usize)> for Picture {
    type Output = bool;
    fn index(&self, (x, y): (usize, usize)) -> &bool {
        self.pixels.index(self.coords_to_index(x, y))
    }
}

impl std::ops::Index<usize> for Picture {
    type Output = [bool];
    fn index(&self, y: usize) -> &[bool] {
        self.pixels.chunks(self.size).nth(y).unwrap()
    }
}

impl std::ops::IndexMut<(usize, usize)> for Picture {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut bool {
        self.pixels.index_mut(self.coords_to_index(x, y))
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let count = Picture::collect_from(
        input
            .trim() // trim to avoid trying to parse the empty space at the end as a tile
            .split("\n\n")
            .map(Tile::lazy_panicky_parse),
    ).into_count();
    println!("{count}");

}

impl Tile {
    fn lazy_panicky_parse(s: &str) -> Self {
        let mut lines = s.lines();
        let id: u16 = lines
            .next()
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .strip_prefix("Tile ")
            .unwrap()
            .parse()
            .unwrap();
        let pixels: [[bool; 10]; 10] = core::array::from_fn(|_| {
            let row = lines.next().unwrap().as_bytes();
            core::array::from_fn(|i| row[i] == b'#')
        });
        Self { id, pixels }
    }

    #[inline(always)]
    const fn top_edge(&self) -> [bool; 10] {
        self.pixels[0]
    }

    #[inline(always)]
    const fn bottom_edge(&self) -> [bool; 10] {
        self.pixels[9]
    }

    fn left_edge(&self) -> [bool; 10] {
        core::array::from_fn(|i| self.pixels[i][0])
    }

    fn right_edge(&self) -> [bool; 10] {
        core::array::from_fn(|i| self.pixels[i][9])
    }

    fn rotate(&mut self) {
        for y in 0..10 {
            for x in y..10 {
                let prev_yx = self.pixels[y][x];
                self.pixels[y][x] = self.pixels[x][y];
                self.pixels[x][y] = prev_yx;
            }
        }
        self.flip();
    }

    fn flip(&mut self) {
        for y in 0..10 {
            self.pixels[y].reverse();
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "Tile {}:", self.id)?;
        for row in self.pixels {
            for pixel in row {
                write!(fmt, "{}", if pixel { '#' } else { '.' })?;
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Picture {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size {
            for pixel in &self[row] {
                write!(fmt, "{}", if *pixel { '#' } else { '.' })?;
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod pic_tests {
    use super::Picture;

    /// Return a Picture with size 3, and the following 3x3 grid for pixels:
    ///
    /// .#.
    /// ..#
    /// ###
    ///
    /// In addition to being a glider from Conway's Game Of Life, it should be easy to make sure
    /// that transformations worked right
    fn test_pic() -> Picture {
        Picture {
            size: 3,
            pixels: vec![
                false, true, false, // .#.
                false, false, true, // ..#
                true, true, true, //   ###
            ],
        }
    }

    // macro to concatenate 3 lines, with a newline after each one.
    // Used to keep a 3x3 grid's string representation visually coherent without unindented lines,
    // and in a form that rustfmt will leave intact (at least at time of writing).
    macro_rules! grid3x3 {
        {$row_1: literal $row_2: literal $row_3: literal } => {{
            concat!($row_1, '\n', $row_2, '\n', $row_3, '\n')
        }}
    }

    // check that `format!("{}", test_pic())` is, in fact, the GOL glider pattern.
    #[test]
    fn sanity_check() {
        const CONWAY_GLIDER: &str = grid3x3! {
            ".#."
            "..#"
            "###"
        };
        assert_eq!(format!("{}", test_pic()), CONWAY_GLIDER);
    }

    // make sure that Picture::flip does, in fact, flip it horizontally
    #[test]
    fn flip_test() {
        let mut pic = test_pic();
        pic.flip();
        const FLIPPED: &str = grid3x3! {
            ".#."
            "#.."
            "###"
        };
        assert_eq!(format!("{pic}"), FLIPPED);
    }

    #[test]
    fn rotate_test() {
        let mut pic = test_pic();
        pic.rotate();
        const ROTATED: &str = grid3x3! {
            "#.."
            "#.#"
            "##."
        };
        assert_eq!(format!("{pic}"), ROTATED, "\n{pic}\n{ROTATED}");
    }

    #[test]
    fn coordinate_pairs() {
        let pic = test_pic();
        assert_eq!(pic.coords_to_index(0, 0), 0);
        assert_eq!(pic.coords_to_index(1, 0), 1);
        assert_eq!(pic.coords_to_index(2, 0), 2);
        assert_eq!(pic.coords_to_index(0, 1), 3);
        assert_eq!(pic.coords_to_index(1, 1), 4);
        assert_eq!(pic.coords_to_index(2, 1), 5);
        assert_eq!(pic.coords_to_index(0, 2), 6);
        assert_eq!(pic.coords_to_index(1, 2), 7);
        assert_eq!(pic.coords_to_index(2, 2), 8);
        // actual index is out-of-bounds, but this should still return
        assert_eq!(pic.coords_to_index(0, 3), 9);
    }

    #[test]
    fn monsters_detected() {
        let pic = Picture {
            size: 24,
            pixels: vec![true; 24 * 24],
        };
        for y in 0..21 {
            for x in 0..4 {
                assert!(
                    pic.contains_monster_at(x, y),
                    "Monster not detected at ({x}, {y}) in all-on 24x24 pixel"
                )
            }
        }

        // the example flipped, rotated image from the problem description that contains 2 monsters
        const EXAMPLE_MONSTER_PIC: &str = concat!(
            ".####...#####..#...###..\n",
            "#####..#..#.#.####..#.#.\n",
            ".#.#...#.###...#.##.##..\n",
            "#.#.##.###.#.##.##.#####\n",
            "..##.###.####..#.####.##\n",
            "...#.#..##.##...#..#..##\n",
            "#.##.#..#.#..#..##.#.#..\n",
            ".###.##.....#...###.#...\n",
            "#.####.#.#....##.#..#.#.\n",
            "##...#..#....#..#...####\n",
            "..#.##...###..#.#####..#\n",
            "....#.##.#.#####....#...\n",
            "..##.##.###.....#.##..#.\n",
            "#...#...###..####....##.\n",
            ".#.##...#.##.#.#.###...#\n",
            "#.###.#..####...##..#...\n",
            "#.###...#.##...#.######.\n",
            ".###.###.#######..#####.\n",
            "..##.#..#..#.#######.###\n",
            "#.#..##.########..#..##.\n",
            "#.#####..#.#...##..#....\n",
            "#....##..#.#########..##\n",
            "#...#.....#..##...###.##\n",
            "#..###....##.#...##.##.#\n",
        );
        let mut pixels = Vec::with_capacity(24 * 24);
        for c in EXAMPLE_MONSTER_PIC.as_bytes() {
            match c {
                b'#' => pixels.push(true),
                b'.' => pixels.push(false),
                b'\n' => (),
                _ => unreachable!(),
            }
        }
        let example_monster_pic = Picture { pixels, size: 24 };
        assert!(
            example_monster_pic.contains_monster_at(2, 2),
            "first monster missed"
        );
        assert!(
            example_monster_pic.contains_monster_at(1, 16),
            "second monster missed"
        );
    }
}
