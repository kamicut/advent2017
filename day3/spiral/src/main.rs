use std::env;
use std::f32;
use std::process;

fn usage() {
    println!("Usage: spiral num");
}

fn find_range(n: u32) -> (u32, u32) {
    let sq = (n as f32).sqrt().floor() as u32;

    if sq % 2 == 0 {
        return ( (sq - 1).pow(2), (sq + 1).pow(2))
    } else {
        return (sq.pow(2), (sq + 2).pow(2));
    }
}

fn distance_to_center(n: u32) -> u32 {
    if n == 1 { return 1 };

    let range = find_range(n);

    // Distance of the range to the center
    let range_dist = ((range.1 as f32).sqrt() / 2.0).floor() as u32;

    // midpoints
    let midpoints = [
        range.1 - range_dist,
        range.1 - range_dist - (range_dist*2),
        range.1 - range_dist - (range_dist*4),
        range.1 - range_dist - (range_dist*6)
    ].to_vec();

    // closest midpoint
    let distances: Vec<u32> = midpoints.iter()
    .map(|x| if n > *x { n - x } else  { x - n })
    .collect();
    let mid_dist = distances.iter().min().unwrap();

    // manhattan distance
    range_dist + mid_dist
}

fn main() {
    let args: Vec<_> = env::args().collect();
    
    if args.len() < 2 {
        usage();
        process::exit(1);
    }

    let input = args[1].parse::<u32>().unwrap();
    let distance = distance_to_center(input);
    println!("Distance: {}", distance);
}