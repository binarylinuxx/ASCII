// src/converter.rs
use anyhow::Result;
use colored::*;
use image::{GenericImageView, Pixel};
use std::fs::File;
use std::io::{self, Write};

use crate::config::{Config, RenderMode};

pub struct AsciiConverter {
    config: Config,
}

impl AsciiConverter {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    
    pub fn convert(&self) -> Result<()> {
        // Open the image
        let img = image::open(&self.config.input_path)?;
        
        // Resize the image
        let img = img.resize_exact(
            self.config.target_width, 
            self.config.target_height, 
            image::imageops::FilterType::Triangle
        );
        
        // Get the symbol pack characters
        let ascii_chars = self.config.symbols.symbols();
        
        // Create the output file
        let mut output = File::create(&self.config.output_path)?;
        
        // Process the image pixel by pixel
        self.process_image(&img, &mut output, ascii_chars)?;
        
        Ok(())
    }
    
    fn process_image<W: Write>(
        &self, 
        img: &image::DynamicImage, 
        output: &mut W,
        ascii_chars: &str
    ) -> io::Result<()> {
        for y in 0..img.height() {
            let mut line = String::new();
            
            for x in 0..img.width() {
                let pixel = img.get_pixel(x, y);
                let channels = pixel.channels();
                
                // Check if the pixel is transparent (assuming RGBA format)
                let is_transparent = channels.len() == 4 && channels[3] < 128;
                
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
                let idx = match self.config.mode {
                    RenderMode::Inverted => ((1.0 - brightness) * (char_count - 1) as f32).round() as usize,
                    _ => (brightness * (char_count - 1) as f32).round() as usize,
                };
                
                // Ensure the index is within bounds
                let safe_idx = idx.min(char_count - 1);
                let symbol = ascii_chars.chars().nth(safe_idx).unwrap_or(' ');
                
                let ascii = match self.config.mode {
                    RenderMode::Grayscale | RenderMode::Inverted => symbol.to_string(),
                    RenderMode::Colorful => format!("{}", symbol.to_string().truecolor(r, g, b)),
                };
                
                line.push_str(&ascii);
            }
            
            // Write to file
            writeln!(output, "{}", line)?;
            
            // Print to console if requested
            if self.config.show_instantly {
                println!("{}", line);
            }
        }
        
        Ok(())
    }
}
