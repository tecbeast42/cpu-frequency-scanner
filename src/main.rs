fn main() {
    let info = include_str!("/proc/cpuinfo");

    let mut count: u64 = 0;
    let mut sum: f64 = 0.0;

    info.lines().filter(|line| {
        line.contains("MHz")
    }).for_each(|line| {
        sum += line.trim_start_matches("cpu MHz\t\t: ")
            .parse::<f64>()
            .expect("Could not parse frequency from cpuinfo");
        count += 1;
    });

    let average: u64 = sum as u64 / count;

    println!("{:?}", average);
}
