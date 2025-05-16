// src/main.rs
mod config;
mod converter;
mod utils;

use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use config::{Config, RenderMode, SymbolPack};
use converter::AsciiConverter;
use std::str::FromStr;

fn main() -> Result<()> {
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
            .action(ArgAction::SetTrue))
        .get_matches();
    
    // Extract command line arguments
    let input_path = matches.get_one::<String>("input")
        .context("Missing input path")?
        .clone();
    
    let output_path = matches.get_one::<String>("output")
        .context("Missing output path")?
        .clone();
    
    let mode_str = matches.get_one::<String>("mode")
        .context("Missing mode")?;
    
    let symbols_str = matches.get_one::<String>("symbols")
        .context("Missing symbols pack")?;
    
    let show_instantly = matches.get_flag("show_instantly");
    
    // Parse mode and symbols
    let mode = RenderMode::from_str(mode_str)
        .context("Invalid render mode")?;
    
    let symbols = SymbolPack::from_str(symbols_str)
        .context("Invalid symbol pack")?;
    
    // Get resolution parameters
    let xs_str = matches.get_one::<String>("xs")
        .context("Missing X resolution")?;
    
    let ys_str = matches.get_one::<String>("ys")
        .context("Missing Y resolution")?;
    
    // Open the image to get original dimensions
    let img = image::open(&input_path)
        .context("Failed to open image")?;
    
    let orig_width = img.width();
    let orig_height = img.height();
    
    // Calculate target dimensions
    let (target_width, target_height) = utils::calculate_dimensions(
        xs_str, 
        ys_str, 
        orig_width, 
        orig_height
    )?;
    
    // Create configuration
    let config = Config {
        input_path,
        output_path,
        mode,
        symbols,
        target_width,
        target_height,
        show_instantly,
    };
    
    // Create converter and process the image
    let converter = AsciiConverter::new(config);
    converter.convert()?;
    
    Ok(())
}
