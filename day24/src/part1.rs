use itertools::Itertools;

#[derive(Debug)]
struct Hail {
    px: isize,
    py: isize,
    _pz: isize,
    vx: isize,
    vy: isize,
    _vz: isize,
}

pub fn execute(input: &str, min_val: f64, max_val: f64) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    inputs.iter().combinations(2).filter(|a| {
        let h1 = a[0];
        let h2 = a[1];
        let (m1, b1) = get_line(h1);
        let (m2, b2) = get_line(h2);

        if m1 == m2 {
            return false;
        }

        let (x0, y0) = intersect(m1, b1, m2, b2);

        if (h1.vx < 0) == (x0 > h1.px as f64) {
            return false;
        }
        if (h2.vx < 0) == (x0 > h2.px as f64) {
            return false;
        }

        x0 >= min_val && x0 <= max_val && y0 >= min_val && y0 <= max_val
    }).count().to_string()
}

fn get_line(h: &Hail) -> (f64, f64) {
    let m = h.vy as f64 / h.vx as f64;
    let b = h.py as f64 - m * h.px as f64;
    (m, b)
}

fn intersect(m1: f64, b1: f64, m2: f64, b2: f64) -> (f64, f64) {
    let x0 = (b2 - b1) / (m1 - m2);
    let y0 = m1 * x0 + b1;

    (x0, y0)
}

fn parseInput(input: &str) -> Vec<Hail> {

    input.lines().map(|line| {
        let split = line.split_once("@").unwrap();
        let positions:Vec<&str> = split.0.split(",").collect();
        let px = positions[0].trim().parse().unwrap();
        let py = positions[1].trim().parse().unwrap();
        let pz = positions[2].trim().parse().unwrap();
        let velocities:Vec<&str> = split.1.split(",").collect();
        let vx = velocities[0].trim().parse().unwrap();
        let vy = velocities[1].trim().parse().unwrap();
        let vz = velocities[2].trim().parse().unwrap();

        Hail{px, py, _pz:pz, vx, vy, _vz:vz}
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
", 7., 27.);
        assert_eq!(result, "2");
    }
}
