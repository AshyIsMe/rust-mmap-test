use mmap_vec::MmapVec;
use rand::Rng;
use std::fs;
use std::time::Instant;

const TARGET_SIZE_BYTES: usize = 1_073_741_824; // 1GB
const I64_SIZE: usize = std::mem::size_of::<i64>();
const NUM_INTEGERS: usize = TARGET_SIZE_BYTES / I64_SIZE; // ~134M integers

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Memory-Mapped Vec<i64> Random Number Generator");
    println!("Target size: {} bytes ({} MB)", TARGET_SIZE_BYTES, TARGET_SIZE_BYTES / (1024 * 1024));
    println!("Number of i64 integers: {}", NUM_INTEGERS);
    println!();

    let file_path = "random_numbers.mmap";

    // Clean up any existing file
    let _ = fs::remove_file(file_path);

    let total_start = Instant::now();

    // Phase 1: Create mmap-backed Vec and generate random numbers
    println!("Phase 1: Creating memory-mapped file and generating random numbers...");
    let gen_start = Instant::now();

    let mut mmap_vec = create_mmap_vec()?;
    generate_random_numbers(&mut mmap_vec)?;

    let gen_duration = gen_start.elapsed();
    println!("Generation completed in {:.2?}", gen_duration);

    // Phase 2: Sum all numbers
    println!("Phase 2: Summing all numbers...");
    let sum_start = Instant::now();

    let total_sum = sum_numbers(&mmap_vec)?;

    let sum_duration = sum_start.elapsed();
    println!("Summation completed in {:.2?}", sum_duration);

    let total_duration = total_start.elapsed();

    // Display results
    println!();
    println!("=== RESULTS ===");
    println!("Total sum: {}", total_sum);
    println!("Generation time: {:.2?}", gen_duration);
    println!("Summation time: {:.2?}", sum_duration);
    println!("Total time: {:.2?}", total_duration);

    // Calculate throughput
    let gen_throughput = (TARGET_SIZE_BYTES as f64) / gen_duration.as_secs_f64() / (1024.0 * 1024.0 * 1024.0);
    let sum_throughput = (TARGET_SIZE_BYTES as f64) / sum_duration.as_secs_f64() / (1024.0 * 1024.0 * 1024.0);

    println!("Generation throughput: {:.2} GB/s", gen_throughput);
    println!("Summation throughput: {:.2} GB/s", sum_throughput);
    println!("Generation rate: {:.0} integers/sec", NUM_INTEGERS as f64 / gen_duration.as_secs_f64());
    println!("Summation rate: {:.0} integers/sec", NUM_INTEGERS as f64 / sum_duration.as_secs_f64());

    // Cleanup
    drop(mmap_vec);
    let _ = fs::remove_file(file_path);

    Ok(())
}

fn create_mmap_vec() -> Result<MmapVec<i64>, Box<dyn std::error::Error>> {
    let mmap_vec = MmapVec::with_capacity(NUM_INTEGERS)?;
    Ok(mmap_vec)
}

fn generate_random_numbers(mmap_vec: &mut MmapVec<i64>) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    for i in 0..NUM_INTEGERS {
        let random_value: i64 = rng.r#gen();
        mmap_vec.push(random_value)?;

        // Progress indicator for every 10M numbers
        if i % 10_000_000 == 0 && i > 0 {
            let progress = (i as f64 / NUM_INTEGERS as f64) * 100.0;
            println!("  Progress: {:.1}% ({}/{})", progress, i, NUM_INTEGERS);
        }
    }

    Ok(())
}

fn sum_numbers(mmap_vec: &MmapVec<i64>) -> Result<i128, Box<dyn std::error::Error>> {
    let mut sum: i128 = 0;
    let len = mmap_vec.len();

    for i in 0..len {
        sum = sum.wrapping_add(mmap_vec[i] as i128);

        // Progress indicator for every 10M numbers
        if i % 10_000_000 == 0 && i > 0 {
            let progress = (i as f64 / len as f64) * 100.0;
            println!("  Progress: {:.1}% ({}/{})", progress, i, len);
        }
    }

    Ok(sum)
}
