mod spigot;
use spigot::Spigot;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut spigot = Spigot::new(10000);
    spigot.find_digits();
        
    let elapsed = start.elapsed();
    println!("Time: {:.3?}", elapsed);
            
    spigot.histogram();
}
