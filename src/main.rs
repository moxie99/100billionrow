use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

/// Calculate the magnitude of the given vector.
// fn magnitude(vector: &[f64; 3]) -> f64 {
//     let mut mag_squared = 0.0;
//     for coord in vector {
//         mag_squared += coord * coord;
//     }
//     mag_squared.sqrt()
// }
/// Change the magnitude of the vector to 1.0 without changing its direction.
// fn normalize(vector: &mut [f64; 3]) {
//     let mag = magnitude(vector);
//     for item in vector {
//         *item /= mag;
//     }
// }

fn main() {
    // let mut point = (1, 2);
    // let mut y_coord: &i32 = &mut point.1;
    // let x_coord: &mut i32 = &mut point.0;

    // let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    // println!("a: {a:?}");
    // let s: &[i32] = &a[2..4];
    // println!("s: {s:?}");
    // println!("shared reference is {}", y_coord);
    // println!("Exclusive reference is {}", x_coord);

    // println!(
    //     "Magnitude of a unit vector: {}",
    //     magnitude(&[0.0, 1.0, 0.0])
    // );
    // let mut v = [1.0, 2.0, 9.0];
    // println!("Magnitude of {v:?}: {}", magnitude(&v));
    // normalize(&mut v);
    // println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));

    // let s1: &str = "Hello";
    // println!("s1: {s1}");
    // let mut s2 = String::from("World");
    // println!("s2: {s2}");
    // s2.push_str(s1);
    // println!("s2: {s2}");

    // let s3 = &s2[2..s2.len()];
    // let s4 = &s1[..];

    // let s5 = &s4[..];
    // println!("s3 {s3}");
    // println!("s4 {s4}");
    // println!("s5 {s5}");

    // println!("{:?}", b"abc");
    // println!("{:?}", &[97, 98, 99]);

    // println!(r#"<a href="link.html">link</a>"#);
    // println!("<a href=\"link.html\">link</a>");

    // let x_ref = {
    //     let x = 10;
    //     &x
    // };
    // dbg!(x_ref);

    //let another_x_coord = &mut point.0;
    //*x_coord = 20;
    //*another_x_coord = 40;
    // println!("point: {point:?}");
    let f = File::open("measurements.txt").unwrap();
    let f = BufReader::new(f);
    let mut stats = HashMap::<String, (f64, f64, usize, f64)>::new();
    for line in f.lines() {
        let line = line.unwrap();
        let (station, temperature) = line.split_once(";").unwrap();
        let temperature: f64 = temperature.parse().unwrap();
        // let stats = stats.entry(station.to_string()).or_default();
        let stats = match stats.get_mut(station) {
            Some(stats) => stats,
            None => stats
                .entry(station.to_string())
                .or_insert((f64::MAX, 0., 0, f64::MIN)),
        };
        stats.0 = stats.0.min(temperature);
        stats.1 += temperature;
        stats.2 += 1;
        stats.3 += stats.3.max(temperature);
    }

    print!("{{");
    let stats = BTreeMap::from_iter(stats);
    let mut stats = stats.into_iter().peekable();
    while let Some((station, (min, sum, count, max))) = stats.next() {
        print!("{station}={min}/{}/{max}", sum / (count as f64));
        if stats.peek().is_some() {
            print!(", ")
        }
    }
    print!("}}");
}
