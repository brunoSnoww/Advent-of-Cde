use glam::{DMat2, DMat4, DVec2, DVec4, Mat2};

use itertools::Itertools as _;

type Pos = (i64, i64, i64);
type V = (i64, i64, i64);
type Hailstone = (Pos, V);
fn find_ts(pos1: Pos, pos2: Pos, v1: V, v2: V) -> DVec2 {
    let M = DMat2::from_cols(
        DVec2::new(v1.0 as f64, v1.1 as f64),
        DVec2::new(-v2.0 as f64, -v2.1 as f64),
    );
    let t = DVec2::new((pos2.0 - pos1.0) as f64, (pos2.1 - pos1.1) as f64);
    M.inverse() * t
}

fn make_row(stone0: Hailstone, stone1: Hailstone) -> DVec4 {
    DVec4::new(
        (stone1.1 .1 - stone0.1 .1) as f64,
        (stone0.1 .0 - stone1.1 .0) as f64,
        (stone0.0 .1 - stone1.0 .1) as f64,
        (stone1.0 .0 - stone0.0 .0) as f64,
    )
}
fn make_rh(stone0: Hailstone, stone1: Hailstone) -> f64 {
    (stone1.0 .0 * stone1.1 .1 - stone0.0 .0 * stone0.1 .1 + stone0.0 .1 * stone0.1 .0
        - stone1.0 .1 * stone1.1 .0) as f64
}

fn main() {
    let input = include_str!("./input24.txt").lines();
    let mut hails: Vec<Hailstone> = Vec::new();
    for l in input {
        let parts: Vec<&str> = l.split("@").collect();
        let p: Vec<&str> = parts[0].split(",").collect();
        let v: Vec<&str> = parts[1].split(",").collect();
        let pos = (
            p[0].trim().parse::<i64>().unwrap(),
            p[1].trim().parse::<i64>().unwrap(),
            p[2].trim().parse::<i64>().unwrap(),
        );
        let v = (
            v[0].trim().parse::<i64>().unwrap(),
            v[1].trim().parse::<i64>().unwrap(),
            v[2].trim().parse::<i64>().unwrap(),
        );
        hails.push((pos, v));
    }
    let L = 200000000000000.0;
    let R = 400000000000000.0;
    let mut ans = 0;
    hails.iter().combinations(2).for_each(|vec| {
        let hails1 = vec[0];
        let hails2 = vec[1];
        let t = find_ts(hails1.0, hails2.0, hails1.1, hails2.1);
        let px = (hails1.0 .0 as f64) + (hails1.1 .0 as f64) * t.x;
        let py = (hails2.0 .1 as f64) + (hails2.1 .1 as f64) * t.y;
        if t.x > 0.0 && t.y > 0.0 && px >= L && px <= R && py >= L && py <= R {
            ans += 1;
        }
    });
    println!("Part1: {ans}");

    //PART2:
    let (h0, h1, h2, h3, h4) = (hails[0], hails[1], hails[2], hails[3], hails[4]);

    let rows = [
        make_row(h0, h1),
        make_row(h1, h2),
        make_row(h2, h3),
        make_row(h3, h4),
    ];
    let M = DMat4::from_cols(rows[0], rows[1], rows[2], rows[3]).transpose();
    let rh = DVec4::new(
        make_rh(h0, h1),
        make_rh(h1, h2),
        make_rh(h2, h3),
        make_rh(h3, h4),
    );

    let sol = M.inverse() * rh;
    let (xs, ys, vxs, vys) = (sol.x, sol.y, sol.z, sol.w);

    let t0 = (h0.0 .0 as f64 - xs) / (vxs - h0.1 .0 as f64);
    let t1 = (h1.0 .0 as f64 - xs) / (vxs - h1.1 .0 as f64);

    let MZ = DMat2::from_cols(DVec2::new(1.0, 1.0), DVec2 { x: t0, y: t1 });
    let rhz = DVec2::new(
        h0.0 .2 as f64 + t0 * (h0.1 .2 as f64),
        h1.0 .2 as f64 + t1 * (h1.1 .2 as f64),
    );

    let solZ = MZ.inverse() * rhz;

    let zs = solZ.x;
    println!("Part2: {}", xs + ys + zs);
}
