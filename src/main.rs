use clap::{Arg, Command};
use colored::*;
use image::{GenericImageView, Pixel};
use std::fs::File;
use std::io::{self, Write};

// Different symbol packs from darkest to lightest
const STANDARD_CHARS: &str = "@%#*+=-:. ";
const DETAILED_CHARS: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
const BLOCKS_CHARS: &str = "█▓▒░ ";
const MINIMAL_CHARS: &str = "#@&%=-:. ";
const DIGITS_CHARS: &str = "9876543210 ";
const BIN_CHARTS: &str = "010101011110";

fn main() -> io::Result<()> {
    let matches = Command::new("ascii")
        .version("1.0")
        .about("Convert images to ASCII art")
        .arg(Arg::new("xs")
            .long("xs")
            .value_name("X_RES")
            .help("Symbol resolution by X (number, 'auto', or 'auto%N' where N is percentage)")
            .required(true))
        .arg(Arg::new("ys")
            .long("ys")
            .value_name("Y_RES")
            .help("Symbol resolution by Y (number, 'auto', or 'auto%N' where N is percentage)")
            .required(true))
        .arg(Arg::new("input")
            .short('i')
            .required(true)
            .value_name("INPUT_IMAGE")
            .help("Input image file path"))
        .arg(Arg::new("output")
            .short('o')
            .required(true)
            .value_name("OUTPUT_FILE")
            .help("Output file path"))
        .arg(Arg::new("mode")
            .short('m')
            .value_name("MODE")
            .required(true)
            .value_parser(["colorful", "grayscale", "inverted"])
            .help("Rendering mode"))
        .arg(Arg::new("symbols")
                .short('s')
                .long("symbols")
                .value_name("SYMBOL_PACK")
                .value_parser(["standard", "detailed", "blocks", "minimal", "digits", "binary"])
                .default_value("standard")
                .help("Symbol pack to use for ASCII conversion"))
        .arg(Arg::new("show_instantly")
                .long("show-instantly")
                .help("Print the ASCII art to console instantly")
                .action(clap::ArgAction::SetTrue))
        .get_matches();
    
    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();
    let mode = matches.get_one::<String>("mode").unwrap();
    let symbols_pack = matches.get_one::<String>("symbols").unwrap();
    let show_instantly = matches.get_flag("show_instantly");
    
    // Select the appropriate symbol pack
    let ascii_chars = match symbols_pack.as_str() {
        "standard" => STANDARD_CHARS,
        "detailed" => DETAILED_CHARS,
        "blocks" => BLOCKS_CHARS,
        "minimal" => MINIMAL_CHARS,
        "digits" => DIGITS_CHARS,
        "binary" => BIN_CHARTS,
        _ => STANDARD_CHARS,
    };
    
    // Open the image
    let img = image::open(input_path).expect("Failed to open image");
    
    // Get original image dimensions
    let orig_width = img.width();
    let orig_height = img.height();
    
    // Get the resolution parameters
    let xs_str = matches.get_one::<String>("xs").unwrap();
    let ys_str = matches.get_one::<String>("ys").unwrap();
    
    // Function to parse auto% values
    fn parse_auto_percentage(value: &str) -> Option<f32> {
        if value.starts_with("auto%") {
            let percentage_str = &value[5..]; // Skip "auto%"
            percentage_str.parse::<f32>().ok().map(|p| p / 100.0)
        } else {
            None
        }
    }

    // Calculate target dimensions
    let (target_width, target_height) = match (xs_str.as_str(), ys_str.as_str()) {
        ("auto", "auto") => {
            // If both are auto, use a reasonable default size (e.g., 80x24)
            (80, 24)
        },
        // Handle auto%N values
        (xs, ys) if xs.starts_with("auto%") && ys.starts_with("auto%") => {
            // Both have percentage scaling
            let xs_percent = parse_auto_percentage(xs).expect("Invalid percentage for --xs");
            let ys_percent = parse_auto_percentage(ys).expect("Invalid percentage for --ys");
            
            // Get console size or use default proportional to image
            let default_width = 80; // Default console width
            let default_height = 24; // Default console height
            
            // Apply percentage to default size
            let width = (default_width as f32 * xs_percent).round() as u32;
            let height = (default_height as f32 * ys_percent).round() as u32;
            
            (width, height)
        },
        (xs, ys_str) if xs.starts_with("auto%") => {
            // Only xs has percentage
            let xs_percent = parse_auto_percentage(xs).expect("Invalid percentage for --xs");
            
            // Parse ys normally
            let ys: u32 = ys_str.parse().expect("Invalid value for --ys");
            
            // Calculate proportional width with percentage scaling
            let aspect_ratio = orig_width as f32 / orig_height as f32;
            let terminal_char_ratio = 0.5; // Typical terminal character width/height ratio
            let xs = (ys as f32 * aspect_ratio / terminal_char_ratio * xs_percent).round() as u32;
            
            (xs, ys)
        },
        (xs_str, ys) if ys.starts_with("auto%") => {
            // Only ys has percentage
            let ys_percent = parse_auto_percentage(ys).expect("Invalid percentage for --ys");
            
            // Parse xs normally
            let xs: u32 = xs_str.parse().expect("Invalid value for --xs");
            
            // Calculate proportional height with percentage scaling
            let aspect_ratio = orig_height as f32 / orig_width as f32;
            let terminal_char_ratio = 0.5; // Typical terminal character width/height ratio
            let ys = (xs as f32 * aspect_ratio * terminal_char_ratio * ys_percent).round() as u32;
            
            (xs, ys)
        },
        ("auto", ys_str) => {
            // If only xs is auto, calculate it based on ys to maintain aspect ratio
            let ys: u32 = ys_str.parse().expect("Invalid value for --ys");
            let aspect_ratio = orig_width as f32 / orig_height as f32;
            // Account for terminal character aspect ratio (characters are typically taller than wide)
            let terminal_char_ratio = 0.5; // Typical terminal character width/height ratio
            let xs = (ys as f32 * aspect_ratio / terminal_char_ratio).round() as u32;
            (xs, ys)
        },
        (xs_str, "auto") => {
            // If only ys is auto, calculate it based on xs to maintain aspect ratio
            let xs: u32 = xs_str.parse().expect("Invalid value for --xs");
            let aspect_ratio = orig_height as f32 / orig_width as f32;
            // Account for terminal character aspect ratio
            let terminal_char_ratio = 0.5; // Typical terminal character width/height ratio
            let ys = (xs as f32 * aspect_ratio * terminal_char_ratio).round() as u32;
            (xs, ys)
        },
        (xs_str, ys_str) => {
            // Both are numeric values
            let xs: u32 = xs_str.parse().expect("Invalid value for --xs");
            let ys: u32 = ys_str.parse().expect("Invalid value for --ys");
            (xs, ys)
        }
    };
    
    // Resize the image
    let img = img.resize_exact(target_width, target_height, image::imageops::FilterType::Triangle);
    
    let mut output = File::create(output_path)?;
    
    for y in 0..img.height() {
        let mut line = String::new();
        
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let channels = pixel.channels();
            
            // Check if the pixel is transparent (assuming RGBA format)
            let is_transparent = channels.len() == 4 && channels[3] < 255;
            
            if is_transparent {
                // For transparent pixels, just output a space
                line.push(' ');
                continue;
            }
            
            let (r, g, b) = (channels[0], channels[1], channels[2]);
            
            // Calculate brightness
            let brightness = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
            
            // Get the total number of characters in the symbol pack
            let char_count = ascii_chars.chars().count();
            
            // Calculate the index within the bounds of available characters
            let idx = match mode.as_str() {
                "inverted" => ((1.0 - brightness) * (char_count - 1) as f32).round() as usize,
                _ => (brightness * (char_count - 1) as f32).round() as usize,
            };
            
            // Ensure the index is within bounds
            let safe_idx = idx.min(char_count - 1);
            let symbol = ascii_chars.chars().nth(safe_idx).unwrap();
            
            let ascii = match mode.as_str() {
                "grayscale" | "inverted" => format!("{}", symbol),
                "colorful" => format!("{}", symbol.to_string().truecolor(r, g, b)),
                _ => unreachable!(),
            };
            
            line.push_str(&ascii);
        }
        
        // Write to file
        writeln!(output, "{}", line)?;
        
        // Print to console if requested
        if show_instantly {
            println!("{}", line);
        }
    }
    
    Ok(())
}
