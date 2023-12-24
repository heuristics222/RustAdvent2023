use itertools::Itertools;

#[derive(Debug)]
struct Hail {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

pub fn execute(input: &str) -> String {
    let hails = parseInput(input.trim());
    // println!("{:#?}", hails);

    let mut sq:usize = 0;
    let mut i_x:usize = 0;
    let mut i_y:usize = 0;
    let scales = vec![(1.,1.), (-1.,1.), (1.,-1.), (-1.,-1.)];

    loop {
        loop {
            for s in &scales {
                let vx = i_x as f64 * s.0;
                let vy = i_y as f64 * s.1;
                let int = intersects_all(&hails, vx, vy);

                if let Some((xt, yt)) = int {
                    let t0 = (xt - hails[0].px) / (hails[0].vx - vx);
                    let t1 = (xt - hails[1].px) / (hails[1].vx - vx);
                    let vr = (hails[0].pz - hails[1].pz + t0 * hails[0].vz - t1 * hails[1].vz) / (t0 - t1);
                    let zt = hails[0].pz + t0 * (hails[0].vz - vr);

                    return (xt + yt + zt).to_string()
                }
            }

            if i_x == sq {
                if i_y == 0 {
                    break;
                }
                i_y -= 1;
            } else {
                i_x += 1;
            }
        }

        sq += 1;
        i_x = 0;
        i_y = sq;
    }
}

fn intersects_all(hails: &Vec<Hail>, vx: f64, vy: f64) -> Option<(f64, f64)> {

    let intersections:Vec<(f64, f64)> = (0..hails.len().min(10)).tuple_windows().map(|(i0, i1)| {
        let h0 = &hails[i0];
        let h1 = &hails[i1];
        let (m0, b0) = get_line(h0.px, h0.vx - vx, h0.py, h0.vy - vy);
        let (m1, b1) = get_line(h1.px, h1.vx - vx, h1.py, h1.vy - vy);

        let (x0, y0) = intersect(m0, b0, m1, b1);

        (x0.round(), y0.round())
    }).collect();

    let comps:Vec<(f64, f64)> = intersections.iter().tuple_windows().filter_map(|((x0, y0),(x1,y1))| {
        if x0 == x1 && y0 == y1 { Some((*x0,*y0)) } else {None}
    }).collect();

    if comps.len() > 1 {
        Some(comps[0])
    } else {
        None
    }
}

fn get_line(p1: f64, v1: f64, p2: f64, v2: f64) -> (f64, f64) {
    let m = v2 / v1;
    let b = p2 - m * p1;
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

        Hail{px, py, pz, vx, vy, vz}
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
");
        assert_eq!(result, "47");
    }
}
