//! The empty state's backdrop.
//!
//! A halftone gradient rising from the bottom of the chat panel: a grid of dots
//! whose *radius* varies and whose colour runs pink -> violet -> blue. Varying
//! the dot size rather than switching cells on and off is what makes it read as
//! print halftone instead of noise.
//!
//! Shown only before the first message. It is a welcome, not a wallpaper.
//!
//! It fades to nothing in the top ~15%, which is what keeps the heading and the
//! composer legible: white text over a lit halftone is a contrast problem, and
//! leaving the upper band black avoids having to solve it.
//!
//! Built once at first use and stretched to the panel.

use std::f32::consts::PI;
use std::fmt::Write;
use std::sync::LazyLock;

/// Spacing between dot centres, in SVG units.
const CELL: f32 = 7.0;
/// The grid the pattern is authored on. It stretches to the real panel, so
/// these are proportions, not pixels.
const COLS: u32 = 110;
const ROWS: u32 = 70;

/// Nothing is drawn above this fraction of the height. Kept low so the glow
/// stays under the composer and suggestions rather than washing across them.
const CLEAR_ABOVE: f32 = 0.42;

/// Largest dot, as a fraction of the cell. Past ~0.5 the dots touch and the
/// halftone becomes a wash.
const MAX_RADIUS: f32 = 0.46;
/// Dots below this are dropped rather than drawn as specks.
const MIN_RADIUS: f32 = 0.35;
/// Peak opacity. The text sits on top of this, so it buys contrast.
const INK: f32 = 0.85;

/// Pastel stops, left to right. Pink, violet, blue - the reference gradient.
const STOPS: [(f32, f32, f32); 3] = [
    (249.0, 168.0, 212.0),
    (167.0, 139.0, 250.0),
    (96.0, 165.0, 250.0),
];

/// The finished SVG. `LazyLock` so the string is built on first use and then
/// reused for every frame - rebuilding it per render would be pointless work.
pub static BACKDROP: LazyLock<String> = LazyLock::new(build);

fn build() -> String {
    let w = COLS as f32 * CELL;
    let h = ROWS as f32 * CELL;

    let mut svg = String::with_capacity(64 * 1024);
    // preserveAspectRatio="none" lets it stretch to the pill's real dimensions
    // instead of keeping the authored ratio and leaving gaps.
    let _ = write!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" preserveAspectRatio="none">"#
    );

    for row in 0..ROWS {
        let down = row as f32 / (ROWS - 1) as f32;

        // Rises from the floor: nothing in the top band, strongest at the
        // bottom edge.
        let vertical = ((down - CLEAR_ABOVE) / (1.0 - CLEAR_ABOVE))
            .max(0.0)
            .powf(1.6);

        for col in 0..COLS {
            let across = col as f32 / (COLS - 1) as f32;

            // Dot size breathes across instead of ramping to nothing. A plain
            // falloff was the first attempt and it killed the dots by ~40%
            // across, where the gradient is still pink - so the whole thing
            // read as a pink smudge rather than pink to blue. Staying well
            // above zero is what lets the colour actually travel.
            let wave = 0.72 + 0.28 * (across * PI * 1.5 + down * 2.2).sin();
            let intensity = wave * vertical;
            let radius = CELL * MAX_RADIUS * intensity;
            if radius < MIN_RADIUS {
                continue;
            }

            // Tilted slightly so the blend runs diagonally, not in columns.
            let (r, g, b) = sample(across * 0.85 + down * 0.15);
            let _ = write!(
                svg,
                r#"<circle cx="{:.1}" cy="{:.1}" r="{radius:.2}" fill="rgb({r},{g},{b})" opacity="{:.3}"/>"#,
                col as f32 * CELL + CELL / 2.0,
                row as f32 * CELL + CELL / 2.0,
                INK * intensity,
            );
        }
    }

    svg.push_str("</svg>");
    svg
}

/// Linear blend through `STOPS` at `t` in 0..1.
fn sample(t: f32) -> (u8, u8, u8) {
    let span = 1.0 / (STOPS.len() - 1) as f32;
    let index = ((t / span) as usize).min(STOPS.len() - 2);
    let local = (t - index as f32 * span) / span;

    let (r0, g0, b0) = STOPS[index];
    let (r1, g1, b1) = STOPS[index + 1];

    (
        (r0 + (r1 - r0) * local) as u8,
        (g0 + (g1 - g0) * local) as u8,
        (b0 + (b1 - b0) * local) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Enough dots to see, not so many it becomes a wash, and nothing in the
    /// top band where the heading sits.
    #[test]
    fn is_visible_and_clears_the_top() {
        let svg = build();
        let dots = svg.matches("<circle").count();
        assert!(dots > 2000, "too sparse to see: {dots} dots");
        assert!(
            dots < (COLS * ROWS) as usize,
            "denser than the grid: {dots}"
        );

        // The top band stays clear so the heading reads against black. The
        // bottom deliberately does not: the glow rises from the floor.
        let top = CELL / 2.0;
        assert!(
            !svg.contains(&format!(r#"cy="{top:.1}""#)),
            "dot in the clear band"
        );
        let bottom = (ROWS - 1) as f32 * CELL + CELL / 2.0;
        assert!(
            svg.contains(&format!(r#"cy="{bottom:.1}""#)),
            "nothing on the bottom edge - the glow should start there"
        );
        assert!(svg.ends_with("</svg>"));
    }

    /// Writes the pattern to /tmp so it can be rendered and looked at:
    ///   cargo test -- --ignored dump
    ///   rsvg-convert -w 760 -h 500 /tmp/backdrop.svg -o /tmp/backdrop.png
    /// Ignored by default - it is a tool, not a check.
    #[test]
    #[ignore]
    fn dump() {
        let preview = build().replacen(
            ">",
            r#"><rect width="100%" height="100%" fill="black"/>"#,
            1,
        );
        std::fs::write("/tmp/backdrop.svg", preview).unwrap();
    }

    /// The gradient must actually travel: left dots pink, right dots blue.
    #[test]
    fn runs_pink_to_blue() {
        let (r, _, b) = sample(0.0);
        assert!(r > b, "left end is not pink");
        let (r, _, b) = sample(1.0);
        assert!(b > r, "right end is not blue");
    }
}
