//! LegatIA: the LegatIA colours over PhotoCraft's tokens (fork Lulindos/photocraft, branch `legatia`).
//! Kept in a file of its own so upstream updates don't touch it: `theme.rs` only calls [`tokens`] when
//! it builds a theme. See LEGATIA.md.
//!
//! The Studio themes are the LegatIA ones (Studio (Dark) is the default): black surfaces with
//! hairline borders, white text in three steps, the brand blue (#006FFE) for accents and selection,
//! and the LegatIA cards' 18 px corners. The other themes (Pro, Classic, Adwaita, ...) stay as they
//! are, for whoever prefers them.
//!
//! Tokens are rewritten with struct update syntax (`..t`): a token upstream adds later keeps
//! PhotoCraft's value until it is mapped here, instead of breaking the build.

use egui::Color32;

use crate::theme::{ThemeKind, Tokens};

/// The LegatIA blue (#006FFE) and the steps of its scale used by the UI (LegatIA design system).
pub const BLUE_300: Color32 = Color32::from_rgb(0x75, 0xB1, 0xFF);
pub const BLUE_400: Color32 = Color32::from_rgb(0x3D, 0x92, 0xFF);
pub const BLUE_500: Color32 = Color32::from_rgb(0x00, 0x6F, 0xFE);
pub const BLUE_600: Color32 = Color32::from_rgb(0x00, 0x59, 0xCC);

const fn rgb(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

/// PhotoCraft's tokens in LegatIA's colours (Studio themes only; any other kind comes back unchanged).
pub fn tokens(t: Tokens) -> Tokens {
    match t.kind {
        ThemeKind::Studio => dark(t),
        ThemeKind::StudioLight => light(t),
        _ => t,
    }
}

fn dark(t: Tokens) -> Tokens {
    Tokens {
        chrome: rgb(0x0A, 0x0A, 0x0B),
        canvas: rgb(0x00, 0x00, 0x00),
        canvas_dot: rgb(0x1C, 0x1C, 0x1F),
        dock: rgb(0x0A, 0x0A, 0x0B),
        card: rgb(0x11, 0x11, 0x13),
        // The LegatIA tokens: hairlines are white at 10% and 16% (--glass-border, -hover), text white and
        // white at 70% and 60% (fg, fg-soft, fg-mute), neutral greys with no tint.
        card_border: Color32::from_white_alpha(26),
        field: Color32::from_white_alpha(15),
        field_border: Color32::from_white_alpha(26),
        hover: Color32::from_white_alpha(20),
        pressed: Color32::from_white_alpha(31),
        text: Color32::WHITE,
        text_dim: rgb(0xB3, 0xB3, 0xB3),
        // At least 3:1 on the cards, fields and selected rows.
        text_faint: rgb(0x99, 0x99, 0x99),
        icon: rgb(0xF5, 0xF5, 0xF5),
        accent: BLUE_500,
        accent_soft: Color32::from_rgba_unmultiplied(0x00, 0x6F, 0xFE, 0x30),
        accent_border: Color32::from_rgba_unmultiplied(0x3D, 0x92, 0xFF, 0x99),
        accent_text: BLUE_300,
        separator: Color32::from_white_alpha(20),
        shadow: Color32::from_black_alpha(160),
        // The primary button is the brand blue with white text.
        primary_bg: BLUE_500,
        primary_text: Color32::WHITE,
        danger: rgb(0xF8, 0x71, 0x71),
        warning: rgb(0xFB, 0x92, 0x3C),
        // The LegatIA cards' corners (18) for the large radius; the small ones follow the system's 6 and 10.
        radius_sm: 6.0,
        radius: 10.0,
        radius_lg: 18.0,
        histogram_bg: rgb(0x0A, 0x0A, 0x0B),
        scrim: Color32::from_black_alpha(140),
        ..t
    }
}

fn light(t: Tokens) -> Tokens {
    Tokens {
        accent: BLUE_500,
        accent_soft: Color32::from_rgba_unmultiplied(0x00, 0x6F, 0xFE, 0x24),
        accent_border: Color32::from_rgba_unmultiplied(0x00, 0x6F, 0xFE, 0x78),
        // 5.9:1 on white: links and labels stay legible.
        accent_text: BLUE_600,
        primary_bg: BLUE_500,
        primary_text: Color32::WHITE,
        radius: 10.0,
        radius_lg: 18.0,
        ..t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_dark_is_black_with_the_brand_blue() {
        let t = Tokens::for_kind(ThemeKind::Studio);
        assert_eq!(t.accent, BLUE_500);
        assert_eq!(t.canvas, Color32::BLACK);
        assert_eq!(t.text, Color32::WHITE);
        assert_eq!(t.radius_lg, 18.0);
    }

    #[test]
    fn studio_light_keeps_its_surfaces_and_takes_the_blue() {
        let light = Tokens::for_kind(ThemeKind::StudioLight);
        assert_eq!(light.accent, BLUE_500);
        assert_eq!(light.accent_text, BLUE_600);
        assert_eq!(light.card, rgb(252, 252, 253));
    }

    #[test]
    fn the_other_themes_are_left_as_they_were() {
        for kind in ThemeKind::ALL {
            if matches!(kind, ThemeKind::Studio | ThemeKind::StudioLight) {
                continue;
            }
            let t = Tokens::for_kind(kind);
            assert_ne!(t.accent, BLUE_500, "{kind:?} should keep its own accent");
        }
    }
}
