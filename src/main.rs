mod dsu;

use dsu::Dsu;
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let map = Map::new_rand(10, 20);
    map.dump();
}

struct Map {
    h: usize,
    w: usize,
    x_fences: Vec<Vec<bool>>,
    y_fences: Vec<Vec<bool>>,
}

impl Map {
    fn new_rand(h: usize, w: usize) -> Map {
        assert!(h > 0 && w > 0);

        struct Edge {
            point: (usize, usize),
            vertical: bool,
        }

        let mut edges = Vec::with_capacity(w * (h - 1) + (w - 1) * h);
        for r in 0..h - 1 {
            for c in 0..w {
                edges.push(Edge {
                    point: (r, c),
                    vertical: true,
                });
            }
        }
        for r in 0..h {
            for c in 0..w - 1 {
                edges.push(Edge {
                    point: (r, c),
                    vertical: false,
                });
            }
        }

        let mut rng = thread_rng();
        edges.shuffle(&mut rng);

        let mut dsu = Dsu::new(w * h);
        let mut x_fences = vec![vec![true; w]; h];
        let mut y_fences = vec![vec![true; w]; h];

        let mut cnt = (((w * h - 1) + (w - 1) * (h - 1)) as f64 / 1.5) as usize;
        for e in edges.iter() {
            let Edge {
                point: (r, c),
                vertical,
            } = *e;
            if vertical {
                let (fa_u, fa_v) = (dsu.find(r * w + c), dsu.find((r + 1) * w + c));
                if fa_u != fa_v {
                    x_fences[r][c] = false;
                    dsu.union(fa_u, fa_v);
                    cnt -= 1;
                }
            } else {
                let (fa_u, fa_v) = (dsu.find(r * w + c), dsu.find(r * w + c + 1));
                if fa_u != fa_v {
                    y_fences[r][c] = false;
                    dsu.union(fa_u, fa_v);
                    cnt -= 1;
                }
            }

            if cnt == 0 {
                break;
            }
        }

        Map {
            w,
            h,
            x_fences,
            y_fences,
        }
    }

    fn dump(&self) {
        print!("+");
        println!("{}", "-+".repeat(self.w));

        for r in 0..self.h {
            print!("|");
            for c in 0..self.w - 1 {
                if self.y_fences[r][c] {
                    print!(" |");
                } else {
                    print!("  ");
                }
            }
            println!(" |");

            print!("+");
            for c in 0..self.w {
                if self.x_fences[r][c] {
                    print!("-+");
                } else {
                    print!(" +");
                }
            }
            println!();
        }
    }
}
