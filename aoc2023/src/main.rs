use std::{fs::File, process::Command, time::Instant};
fn main() {
    let now = Instant::now();
    Command::new("cargo")
        .args(["run", "--release", "--bin", "00"])
        .output()
        .unwrap();
    let baseline_time = now.elapsed();
    println!("Baseline time: {:2?}", baseline_time);

    let total_time = (1..=25)
        .map(|day_num| {
            let day = format!("{:0>2}", day_num);
            let now = Instant::now();
            let cmd = Command::new("cargo")
                .args(["run", "--release", "--bin", &day])
                .stdin(File::open(format!("inputs/{day}.in")).unwrap())
                .output()
                .unwrap();
            let output = String::from_utf8(cmd.stdout).unwrap();
            let elapsed = now.elapsed();
            println!("Day {}:\n{:2?}\n{}", day, elapsed, output);
            //extract_time(&output)
            elapsed.as_millis() as u32
        })
        .sum::<u32>();
    println!("Total time: {}ms", total_time);
}
