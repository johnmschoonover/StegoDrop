use image::{GenericImageView, ImageBuffer, Rgba};
use std::f64::consts::PI;

pub struct StegoEngine;

// Standard JPEG Quantization Table (Luminance) - simplified or standard
const QUANTIZATION_TABLE: [[f64; 8]; 8] = [
    [16.0, 11.0, 10.0, 16.0, 24.0, 40.0, 51.0, 61.0],
    [12.0, 12.0, 14.0, 19.0, 26.0, 58.0, 60.0, 55.0],
    [14.0, 13.0, 16.0, 24.0, 40.0, 57.0, 69.0, 56.0],
    [14.0, 17.0, 22.0, 29.0, 51.0, 87.0, 80.0, 62.0],
    [18.0, 22.0, 37.0, 56.0, 68.0, 109.0, 103.0, 77.0],
    [24.0, 35.0, 55.0, 64.0, 81.0, 104.0, 113.0, 92.0],
    [49.0, 64.0, 78.0, 87.0, 103.0, 121.0, 120.0, 101.0],
    [72.0, 92.0, 95.0, 98.0, 112.0, 100.0, 103.0, 99.0],
];

// Coordinate for embedding (Mid-band)
const EMBED_U: usize = 4;
const EMBED_V: usize = 3;

impl StegoEngine {

    /// Embeds a byte slice into an image using DCT steganography.
    /// Returns the modified image buffer.
    /// Note: Capacity is (Width/8 * Height/8) bits.
    pub fn embed_message(img: &image::DynamicImage, data: &[u8]) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
        let (width, height) = img.dimensions();
        let mut output = img.to_rgba8();

        // Convert data to bits
        let mut bits: Vec<u8> = Vec::new();
        // Prefix length (32-bit integer)
        let len = data.len() as u32;
        for i in 0..32 {
            bits.push(((len >> i) & 1) as u8);
        }
        // Data bits
        for byte in data {
            for i in 0..8 {
                bits.push((byte >> i) & 1);
            }
        }

        let total_blocks = (width / 8) * (height / 8);
        if bits.len() as u32 > total_blocks {
            return Err(format!("Image too small. Capacity: {} bits, Needed: {} bits", total_blocks, bits.len()));
        }

        let mut bit_idx = 0;

        for y in (0..height - 7).step_by(8) {
            for x in (0..width - 7).step_by(8) {
                if bit_idx >= bits.len() {
                    break;
                }

                let bit_to_embed = bits[bit_idx];

                // 1. Extract 8x8 block (Y channel only)
                let mut block = [[0.0; 8]; 8];
                for by in 0..8 {
                    for bx in 0..8 {
                        let pixel = output.get_pixel(x + bx, y + by);
                        // Simple Luminance conversion: 0.299R + 0.587G + 0.114B
                        let r = pixel[0] as f64;
                        let g = pixel[1] as f64;
                        let b = pixel[2] as f64;
                        block[by as usize][bx as usize] = 0.299 * r + 0.587 * g + 0.114 * b - 128.0;
                    }
                }

                // 2. DCT
                let mut dct_block = dct_2d(&block);

                // 3. Embed Bit in Mid-Band
                // Quantize specifically the target coeff to make modifying it robust-ish
                let q = QUANTIZATION_TABLE[EMBED_V][EMBED_U];
                let coeff = dct_block[EMBED_V][EMBED_U];
                let quantized = (coeff / q).round() as i32;

                // LSB embedding on the quantized coefficient
                let mut new_quantized = quantized;
                if (new_quantized.abs() % 2) != (bit_to_embed as i32) {
                    if new_quantized >= 0 {
                        new_quantized += 1;
                    } else {
                        new_quantized -= 1;
                    }
                }

                // De-quantize (apply change)
                dct_block[EMBED_V][EMBED_U] = new_quantized as f64 * q;

                // 4. IDCT
                let idct_block = idct_2d(&dct_block);

                // 5. Write back
                for by in 0..8 {
                    for bx in 0..8 {
                        let original_pixel = output.get_pixel(x + bx, y + by);
                        let old_y = 0.299 * original_pixel[0] as f64 + 0.587 * original_pixel[1] as f64 + 0.114 * original_pixel[2] as f64;
                        let new_y = idct_block[by as usize][bx as usize] + 128.0;
                        let diff = new_y - old_y;

                        // Distribute error to RGB to preserve color balance but match new Luminance
                        let r = (original_pixel[0] as f64 + diff).clamp(0.0, 255.0) as u8;
                        let g = (original_pixel[1] as f64 + diff).clamp(0.0, 255.0) as u8;
                        let b = (original_pixel[2] as f64 + diff).clamp(0.0, 255.0) as u8;

                        output.put_pixel(x + bx, y + by, Rgba([r, g, b, original_pixel[3]]));
                    }
                }

                bit_idx += 1;
            }
        }

        Ok(output)
    }

    pub fn extract_message(img: &image::DynamicImage) -> Result<Vec<u8>, String> {
        let (width, height) = img.dimensions();
        let img_buf = img.to_rgba8();

        let mut bits: Vec<u8> = Vec::new();

        // We don't know length yet, so we read 32 bits first
        let mut len_bits: Vec<u8> = Vec::new();
        let mut reading_len = true;
        let mut msg_len: u32 = 0;
        let mut bit_count = 0;

        for y in (0..height - 7).step_by(8) {
            for x in (0..width - 7).step_by(8) {

                // Extract 8x8 block Y
                let mut block = [[0.0; 8]; 8];
                for by in 0..8 {
                    for bx in 0..8 {
                        let pixel = img_buf.get_pixel(x + bx, y + by);
                        let r = pixel[0] as f64;
                        let g = pixel[1] as f64;
                        let b = pixel[2] as f64;
                        block[by as usize][bx as usize] = 0.299 * r + 0.587 * g + 0.114 * b - 128.0;
                    }
                }

                let dct_block = dct_2d(&block);

                let q = QUANTIZATION_TABLE[EMBED_V][EMBED_U];
                let coeff = dct_block[EMBED_V][EMBED_U];
                let quantized = (coeff / q).round() as i32;

                let bit = (quantized.abs() % 2) as u8;

                if reading_len {
                    len_bits.push(bit);
                    if len_bits.len() == 32 {
                        // Reconstruct length
                        for i in 0..32 {
                            if len_bits[i] == 1 {
                                msg_len |= 1 << i;
                            }
                        }
                        reading_len = false;
                        // Reset to read body
                    }
                } else {
                    bits.push(bit);
                    bit_count += 1;
                    if bit_count >= (msg_len * 8) {
                         // Done
                         return Ok(bits_to_bytes(&bits));
                    }
                }
            }
        }

        if reading_len {
            Err("Failed to read length prefix".to_string())
        } else if bit_count < msg_len * 8 {
            Err("Image unexpected ended before message complete".to_string())
        } else {
            Ok(bits_to_bytes(&bits))
        }
    }
}

fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for chunk in bits.chunks(8) {
        let mut byte = 0;
        for (i, &bit) in chunk.iter().enumerate() {
            if bit == 1 {
                byte |= 1 << i;
            }
        }
        bytes.push(byte);
    }
    bytes
}

// Basic DCT-II Implementation
// C(u) = 1/sqrt(2) if u=0, else 1
fn c(u: usize) -> f64 {
    if u == 0 { 1.0 / (2.0f64).sqrt() } else { 1.0 }
}

fn dct_2d(input: &[[f64; 8]; 8]) -> [[f64; 8]; 8] {
    let mut output = [[0.0; 8]; 8];
    for u in 0..8 {
        for v in 0..8 {
            let mut sum = 0.0;
            for x in 0..8 {
                for y in 0..8 {
                    sum += input[x][y] *
                           ((2.0 * x as f64 + 1.0) * u as f64 * PI / 16.0).cos() *
                           ((2.0 * y as f64 + 1.0) * v as f64 * PI / 16.0).cos();
                }
            }
            output[u][v] = 0.25 * c(u) * c(v) * sum;
        }
    }
    output
}

fn idct_2d(input: &[[f64; 8]; 8]) -> [[f64; 8]; 8] {
    let mut output = [[0.0; 8]; 8];
    for x in 0..8 {
        for y in 0..8 {
            let mut sum = 0.0;
            for u in 0..8 {
                for v in 0..8 {
                    sum += c(u) * c(v) * input[u][v] *
                           ((2.0 * x as f64 + 1.0) * u as f64 * PI / 16.0).cos() *
                           ((2.0 * y as f64 + 1.0) * v as f64 * PI / 16.0).cos();
                }
            }
            output[x][y] = 0.25 * sum;
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};

    #[test]
    fn test_dct_stego_loop() {
        // Create a simple random image
        let width = 64;
        let height = 64;
        let mut img = RgbaImage::new(width, height);
        for pixel in img.pixels_mut() {
            *pixel = Rgba([100, 150, 200, 255]); // Flat color for simplicity
        }
        let dynamic_img = DynamicImage::ImageRgba8(img);

        let message = b"TEST";
        let embedded_buf = StegoEngine::embed_message(&dynamic_img, message).expect("Embedding failed");
        let embedded_img = DynamicImage::ImageRgba8(embedded_buf);

        let extracted = StegoEngine::extract_message(&embedded_img).expect("Extraction failed");

        assert_eq!(message.to_vec(), extracted);
    }
}
