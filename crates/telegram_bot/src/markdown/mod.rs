pub mod issuers;
pub mod politicians;
pub mod trades;

fn format_volume(n: i64) -> String {
    if n < 1_000 {
        return format!("{}", n);
    }
    if n < 1_000_000 {
        return format!("{:.1}k", n as f32 / 1_000.0);
    }
    if n < 1_000_000_000 {
        return format!("{:.1}M", n as f32 / 1_000_000.0);
    }
    format!("{:.1}B", n as f32 / 1_000_000_000.0)
}
