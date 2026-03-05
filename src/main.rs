mod config;
mod landscape;
mod spigot;
use landscape::Landscape;
use spigot::Spigot;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut spigot = Spigot::new(config::NUM_PI_DIGITS);
    spigot.find_digits();

    let mut landscape = Landscape::new(spigot.result);
    landscape.filter_landscape();    
    landscape.plot_landscape();

    let elapsed = start.elapsed();
    println!("Time: {:.3?}", elapsed);
}
