use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn generate_hsv_lut(_input: TokenStream) -> TokenStream {
    let mut data = Vec::new();

    // Generate 256 hue × 8 sat × 8 val = 16,384 entries
    for hue_idx in 0..256 {
        for sat_idx in 0..8 {
            for val_idx in 0..8 {
                let h = (hue_idx * 360) / 256;
                let s = (sat_idx * 255) / 7;
                let v = (val_idx * 255) / 7;

                let pixel = hsv_to_rgb(h, s, v);
                data.push(pixel);
            }
        }
    }

    // Generate array literal
    let pixels = data.iter().map(|(r, g, b)| {
        quote! { Pixel::new(#r, #g, #b) }
    });

    let expanded = quote! {
        [#(#pixels),*]
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn generate_gamma_lut(_input: TokenStream) -> TokenStream {
    let gamma = 2.2f64;
    let mut data = Vec::new();

    for i in 0..256 {
        let normalized = i as f64 / 255.0;
        let corrected = normalized.powf(gamma);
        let value = (corrected * 255.0).round() as u8;
        data.push(value);
    }

    let expanded = quote! {
        [#(#data),*]
    };

    TokenStream::from(expanded)
}

// Helper function for HSV to RGB conversion
fn hsv_to_rgb(h: u32, s: u32, v: u32) -> (u8, u8, u8) {
    if s == 0 {
        return (v as u8, v as u8, v as u8);
    }

    let region = (h / 60) as u32;
    let remainder = ((h % 60) * 255) / 60;

    let p = (v * (255 - s)) / 255;
    let q = (v * (255 - ((s * remainder) / 255))) / 255;
    let t = (v * (255 - ((s * (255 - remainder)) / 255))) / 255;

    match region {
        0 => (v as u8, t as u8, p as u8),
        1 => (q as u8, v as u8, p as u8),
        2 => (p as u8, v as u8, t as u8),
        3 => (p as u8, q as u8, v as u8),
        4 => (t as u8, p as u8, v as u8),
        _ => (v as u8, p as u8, q as u8),
    }
}
