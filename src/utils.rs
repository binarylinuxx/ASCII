// src/utils.rs
use anyhow::{anyhow, Result};

// Typical terminal character width/height ratio
pub const TERMINAL_CHAR_RATIO: f32 = 0.5;

// Function to parse auto% values
pub fn parse_auto_percentage(value: &str) -> Option<f32> {
    if value.starts_with("auto%") {
        let percentage_str = &value[5..]; // Skip "auto%"
        percentage_str.parse::<f32>().ok().map(|p| p / 100.0)
    } else {
        None
    }
}

// Calculate the target dimensions for the ASCII art based on the resolution parameters
pub fn calculate_dimensions(
    xs_str: &str,
    ys_str: &str,
    orig_width: u32,
    orig_height: u32,
) -> Result<(u32, u32)> {
    
    match (xs_str, ys_str) {
        ("auto", "auto") => {
            // If both are auto, use a reasonable default size (e.g., 80x24)
            Ok((80, 24))
        },
        // Handle auto%N values
        (xs, ys) if xs.starts_with("auto%") && ys.starts_with("auto%") => {
            // Both have percentage scaling
            let xs_percent = parse_auto_percentage(xs).ok_or_else(|| anyhow!("Invalid percentage for --xs"))?;
            let ys_percent = parse_auto_percentage(ys).ok_or_else(|| anyhow!("Invalid percentage for --ys"))?;
            
            // Get console size or use default proportional to image
            let default_width = 80; // Default console width
            let default_height = 24; // Default console height
            
            // Apply percentage to default size
            let width = (default_width as f32 * xs_percent).round() as u32;
            let height = (default_height as f32 * ys_percent).round() as u32;
            
            Ok((width, height))
        },
        (xs, ys_str) if xs.starts_with("auto%") => {
            // Only xs has percentage
            let xs_percent = parse_auto_percentage(xs)
                .ok_or_else(|| anyhow!("Invalid percentage for --xs"))?;
            
            // Parse ys normally
            let ys: u32 = ys_str.parse()
                .map_err(|_| anyhow!("Invalid value for --ys"))?;
            
            // Calculate proportional width with percentage scaling
            let aspect_ratio = orig_width as f32 / orig_height as f32;
            let xs = (ys as f32 * aspect_ratio / TERMINAL_CHAR_RATIO * xs_percent).round() as u32;
            
            Ok((xs, ys))
        },
        (xs_str, ys) if ys.starts_with("auto%") => {
            // Only ys has percentage
            let ys_percent = parse_auto_percentage(ys)
                .ok_or_else(|| anyhow!("Invalid percentage for --ys"))?;
            
            // Parse xs normally
            let xs: u32 = xs_str.parse()
                .map_err(|_| anyhow!("Invalid value for --xs"))?;
            
            // Calculate proportional height with percentage scaling
            let aspect_ratio = orig_height as f32 / orig_width as f32;
            let ys = (xs as f32 * aspect_ratio * TERMINAL_CHAR_RATIO * ys_percent).round() as u32;
            
            Ok((xs, ys))
        },
        ("auto", ys_str) => {
            // If only xs is auto, calculate it based on ys to maintain aspect ratio
            let ys: u32 = ys_str.parse()
                .map_err(|_| anyhow!("Invalid value for --ys"))?;
            let aspect_ratio = orig_width as f32 / orig_height as f32;
            // Account for terminal character aspect ratio (characters are typically taller than wide)
            let xs = (ys as f32 * aspect_ratio / TERMINAL_CHAR_RATIO).round() as u32;
            Ok((xs, ys))
        },
        (xs_str, "auto") => {
            // If only ys is auto, calculate it based on xs to maintain aspect ratio
            let xs: u32 = xs_str.parse()
                .map_err(|_| anyhow!("Invalid value for --xs"))?;
            let aspect_ratio = orig_height as f32 / orig_width as f32;
            // Account for terminal character aspect ratio
            let ys = (xs as f32 * aspect_ratio * TERMINAL_CHAR_RATIO).round() as u32;
            Ok((xs, ys))
        },
        (xs_str, ys_str) => {
            // Both are numeric values
            let xs: u32 = xs_str.parse()
                .map_err(|_| anyhow!("Invalid value for --xs"))?;
            let ys: u32 = ys_str.parse()
                .map_err(|_| anyhow!("Invalid value for --ys"))?;
            Ok((xs, ys))
        }
    }
}
