fn main() {
    let f = include_str!("/proc/cpuinfo");

    let frequencies = f.lines().filter(|line| {
        line.contains("MHz")
    }).map(|line| {
        line.trim_start_matches("cpu MHz\t\t: ")
            .parse::<f64>()
            .expect("Could not parse frequency from cpuinfo")
    });

    let sum: f64 = frequencies.clone().sum();
    let count = frequencies.count() as f64;
    let average: u64 = (sum / count) as u64;

    println!("{:?}", average);
}
