use std::{self, process, env};
use std::fs::{File, metadata};
use std::io::prelude::*;
use std::io::Read;

const HEADER_SIZE: usize = 44;

fn main() {
    // Get args from user, and store them in variables with according types
    let args = get_args(
        "Usage : cargo run [file_input.wav] [file_output.wav]", 3, 3
    );
    let path = String::from("");
    let input_file_name = &args[0];
    let output_file_name = &args[1];
    let factor = string_to_float(&args[2]);

    // Read the input files, and separate the header from the data
    let input_file: Vec<u8> = get_file_as_byte_vec(&path, &input_file_name);
    let header: Vec<u8> = split_file(&input_file, 0, HEADER_SIZE);
    let aheader = &header[..];
    let mut file_data: Vec<u8> = split_file(&input_file, HEADER_SIZE, input_file.len());
    
    // Multiply the data with the factor, then change the type
    multiply(&mut file_data, factor);
    let afile_data = &file_data[..];

    // Create or open as writable the output file and write the header and the updated datas
    let mut output_file = File::create(path.clone() + &output_file_name.clone()).expect("Could not create file.");
    output_file.write(&aheader).expect("Something went wrong writing the header.");
    output_file.write(&afile_data).expect("Something went wrong wrong writing the data.");
}

// Returns a Vec<String> with the user entries (drop the [0])
fn get_args(
    info: &str,
    min: usize,
    max: usize
) -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() < min || args.len() > max {
        println!("{}", info);
        process::exit(1);
    }
    args
}

// Transform a string into an unsigned int, or exit if failed. 
fn string_to_float(nb_str: &str) -> f32 {
    let nb: f32 = match nb_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please provide a correct number.");
            process::exit(1);
        },
    };
    nb
}

// Read a file, and store and return its code as byte in a Vec<u8>
fn get_file_as_byte_vec(
    path: &String,
    filename: &String
) -> Vec<u8> {
    let full_path = path.clone() + &filename.clone();
    let mut f = File::open(&full_path).expect("No file found");
    let metadata = metadata(&full_path).expect("Unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer overflow");

    buffer
}

// Return a slice of a vector as an other vector
fn split_file(
    file: &Vec<u8>,
    start: usize,
    end: usize
) -> Vec<u8> {
    let mut header: Vec<u8> = vec![];
    for i in start..end {
        header.push(file[i]);
    }
    header
}

// Multiplies pairs of bytes
fn multiply(data: &mut Vec<u8>, factor: f32) {
    for i in 0..data.len() {
        if i % 2 == 0 {
            let sample = ((data[i] as i16) << 8) | data[i + 1] as i16;
            let fsample: f32 = sample as f32 * factor;
            let isample: i16 = fsample as i16;
            let sample_byte = isample.to_ne_bytes();
            data[i] = sample_byte[1] as u8;
            data[i + 1] = sample_byte[0] as u8;
        }
    }
}
