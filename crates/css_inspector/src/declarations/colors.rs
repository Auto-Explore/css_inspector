use crate::report::{Report, push_error};
use crate::strutil::ascii_lowercase_cow;

use super::{
    is_balanced_function_call_token, is_css_wide_keyword, is_property_ident_char, parse_css_number,
    unescape_css_identifier_css1_compat, unescape_css_identifier_greedy,
};

pub(super) fn validate_color(
    tokens: &[&str],
    css1_escapes: bool,
    lenient: bool,
    report: &mut Report,
) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “color”.");
        return;
    };
    let v = token.trim();
    if !is_valid_color_token(v, css1_escapes, lenient) {
        push_error(report, "Invalid value for property “color”.");
    }
}

pub(super) fn validate_background_color(
    tokens: &[&str],
    css1_escapes: bool,
    lenient: bool,
    report: &mut Report,
) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “background-color”.");
        return;
    };
    let v = token.trim();
    if !is_valid_color_token(v, css1_escapes, lenient) {
        push_error(report, "Invalid value for property “background-color”.");
    }
}

pub(super) fn validate_border_side_color(
    tokens: &[&str],
    css1_escapes: bool,
    lenient: bool,
    report: &mut Report,
) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “border-*-color”.");
        return;
    };
    let v = token.trim();
    if !is_valid_color_token(v, css1_escapes, lenient) {
        push_error(report, "Invalid value for property “border-*-color”.");
    }
}

pub(super) fn validate_outline_color(
    tokens: &[&str],
    css1_escapes: bool,
    css4_profile: bool,
    lenient: bool,
    report: &mut Report,
) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “outline-color”.");
        return;
    };
    let v = token.trim();
    if v.eq_ignore_ascii_case("invert")
        || (css4_profile && v.eq_ignore_ascii_case("auto"))
        || is_valid_color_token(v, css1_escapes, lenient)
    {
        return;
    }
    push_error(report, "Invalid value for property “outline-color”.");
}

fn is_hex_color(s: &str) -> bool {
    let Some(hex) = s.strip_prefix('#') else {
        return false;
    };
    matches!(hex.len(), 3 | 4 | 6 | 8) && hex.chars().all(|c| c.is_ascii_hexdigit())
}

pub(super) fn is_valid_color_token(raw: &str, css1_escapes: bool, lenient: bool) -> bool {
    let t = raw.trim();
    if t.is_empty() {
        return false;
    }
    if t.ends_with('\\') {
        // Trailing escapes are treated as invalid in the autotest suite (bug 3631).
        return false;
    }
    // Strings are not colors.
    if t.len() >= 2 {
        let bytes = t.as_bytes();
        let q = bytes[0];
        if (q == b'"' || q == b'\'') && bytes[bytes.len() - 1] == q {
            return false;
        }
    }

    let lower_ascii = ascii_lowercase_cow(t);
    let lower_ascii = lower_ascii.as_ref();

    // Hex colors.
    if is_hex_color(lower_ascii) {
        return true;
    }

    // Functional colors.
    if lower_ascii.starts_with("rgb(") {
        return is_valid_rgb_like_function(lower_ascii, false, lenient);
    }
    if lower_ascii.starts_with("rgba(") {
        return is_valid_rgb_like_function(lower_ascii, true, lenient);
    }
    if lower_ascii.starts_with("hsl(") {
        return is_valid_hsl_like_function(lower_ascii, false, lenient);
    }
    if lower_ascii.starts_with("hsla(") {
        return is_valid_hsl_like_function(lower_ascii, true, lenient);
    }
    if lower_ascii.starts_with("oklch(") {
        return is_balanced_function_call_token(lower_ascii, "oklch");
    }
    if lower_ascii.starts_with("oklab(") {
        return is_balanced_function_call_token(lower_ascii, "oklab");
    }
    if lower_ascii.starts_with("hwb(") {
        return is_balanced_function_call_token(lower_ascii, "hwb");
    }
    if lower_ascii.starts_with("lab(") {
        return is_balanced_function_call_token(lower_ascii, "lab");
    }
    if lower_ascii.starts_with("lch(") {
        return is_balanced_function_call_token(lower_ascii, "lch");
    }
    if lower_ascii.starts_with("color-mix(") {
        return is_balanced_function_call_token(lower_ascii, "color-mix");
    }
    if lower_ascii.starts_with("color(") {
        return is_balanced_function_call_token(lower_ascii, "color");
    }
    if lower_ascii.starts_with("device-cmyk(") {
        return is_balanced_function_call_token(lower_ascii, "device-cmyk");
    }
    if lower_ascii.starts_with("light-dark(") {
        return is_balanced_function_call_token(lower_ascii, "light-dark");
    }
    if lower_ascii.starts_with("color-contrast(") {
        return is_balanced_function_call_token(lower_ascii, "color-contrast");
    }
    if lower_ascii.starts_with("contrast-color(") {
        return is_balanced_function_call_token(lower_ascii, "contrast-color");
    }
    if lower_ascii.starts_with("var(") {
        return is_balanced_function_call_token(lower_ascii, "var");
    }
    if lower_ascii.starts_with("env(") {
        return is_balanced_function_call_token(lower_ascii, "env");
    }
    if lenient && lower_ascii.starts_with("attr(") {
        return is_balanced_function_call_token(lower_ascii, "attr");
    }
    if lenient && lower_ascii.starts_with("if(") {
        return is_balanced_function_call_token(lower_ascii, "if");
    }
    if lenient && lower_ascii.starts_with("--") && lower_ascii.ends_with(')')
        && let Some(open) = lower_ascii.find('(') {
            let name = lower_ascii[..open].trim();
            if name.starts_with("--") && name.as_bytes().iter().all(|&b| is_property_ident_char(b))
            {
                return true;
            }
        }

    // Ident colors (with CSS escapes).
    let ident = if css1_escapes {
        unescape_css_identifier_css1_compat(t)
    } else {
        unescape_css_identifier_greedy(t)
    };
    let ident_lower = ascii_lowercase_cow(&ident);
    let ident_lower = ident_lower.as_ref();
    ident_lower == "transparent"
        || ident_lower == "currentcolor"
        || is_basic_named_color(ident_lower)
        || (lenient && is_system_color_keyword(ident_lower))
        || is_css_wide_keyword(ident_lower)
}

fn is_system_color_keyword(t: &str) -> bool {
    matches!(
        t,
        // CSS Color 4 system colors (plus legacy CSS2 names still used in some test suites).
        "accentcolor"
            | "accentcolortext"
            | "activetext"
            | "buttonborder"
            | "buttonface"
            | "buttontext"
            | "canvas"
            | "canvastext"
            | "field"
            | "fieldtext"
            | "graytext"
            | "highlight"
            | "highlighttext"
            | "linktext"
            | "mark"
            | "marktext"
            | "selecteditem"
            | "selecteditemtext"
            | "visitedtext"
            | "activeborder"
            | "activecaption"
            | "appworkspace"
            | "background"
            | "buttonhighlight"
            | "buttonshadow"
            | "captiontext"
            | "inactiveborder"
            | "inactivecaption"
            | "inactivecaptiontext"
            | "infobackground"
            | "infotext"
            | "menu"
            | "menutext"
            | "scrollbar"
            | "threeddarkshadow"
            | "threedface"
            | "threedhighlight"
            | "threedlightshadow"
            | "threedshadow"
            | "window"
            | "windowframe"
            | "windowtext"
    )
}

fn is_basic_named_color(t: &str) -> bool {
    matches!(
        t,
        // CSS named colors (including legacy X11 names and common synonyms like `cyan`/`magenta`).
        "aliceblue"
            | "antiquewhite"
            | "aqua"
            | "aquamarine"
            | "azure"
            | "beige"
            | "bisque"
            | "black"
            | "blanchedalmond"
            | "blue"
            | "blueviolet"
            | "brown"
            | "burlywood"
            | "cadetblue"
            | "chartreuse"
            | "chocolate"
            | "coral"
            | "cornflowerblue"
            | "cornsilk"
            | "crimson"
            | "cyan"
            | "darkblue"
            | "darkcyan"
            | "darkgoldenrod"
            | "darkgray"
            | "darkgreen"
            | "darkgrey"
            | "darkkhaki"
            | "darkmagenta"
            | "darkolivegreen"
            | "darkorange"
            | "darkorchid"
            | "darkred"
            | "darksalmon"
            | "darkseagreen"
            | "darkslateblue"
            | "darkslategray"
            | "darkslategrey"
            | "darkturquoise"
            | "darkviolet"
            | "deeppink"
            | "deepskyblue"
            | "dimgray"
            | "dimgrey"
            | "dodgerblue"
            | "firebrick"
            | "floralwhite"
            | "forestgreen"
            | "fuchsia"
            | "gainsboro"
            | "ghostwhite"
            | "gold"
            | "goldenrod"
            | "gray"
            | "green"
            | "greenyellow"
            | "grey"
            | "honeydew"
            | "hotpink"
            | "indianred"
            | "indigo"
            | "ivory"
            | "khaki"
            | "lavender"
            | "lavenderblush"
            | "lawngreen"
            | "lemonchiffon"
            | "lightblue"
            | "lightcoral"
            | "lightcyan"
            | "lightgoldenrodyellow"
            | "lightgray"
            | "lightgreen"
            | "lightgrey"
            | "lightpink"
            | "lightsalmon"
            | "lightseagreen"
            | "lightskyblue"
            | "lightslategray"
            | "lightslategrey"
            | "lightsteelblue"
            | "lightyellow"
            | "lime"
            | "limegreen"
            | "linen"
            | "magenta"
            | "maroon"
            | "mediumaquamarine"
            | "mediumblue"
            | "mediumorchid"
            | "mediumpurple"
            | "mediumseagreen"
            | "mediumslateblue"
            | "mediumspringgreen"
            | "mediumturquoise"
            | "mediumvioletred"
            | "midnightblue"
            | "mintcream"
            | "mistyrose"
            | "moccasin"
            | "navajowhite"
            | "navy"
            | "oldlace"
            | "olive"
            | "olivedrab"
            | "orange"
            | "orangered"
            | "orchid"
            | "palegoldenrod"
            | "palegreen"
            | "paleturquoise"
            | "palevioletred"
            | "papayawhip"
            | "peachpuff"
            | "peru"
            | "pink"
            | "plum"
            | "powderblue"
            | "purple"
            | "rebeccapurple"
            | "red"
            | "rosybrown"
            | "royalblue"
            | "saddlebrown"
            | "salmon"
            | "sandybrown"
            | "seagreen"
            | "seashell"
            | "sienna"
            | "silver"
            | "skyblue"
            | "slateblue"
            | "slategray"
            | "slategrey"
            | "snow"
            | "springgreen"
            | "steelblue"
            | "tan"
            | "teal"
            | "thistle"
            | "tomato"
            | "turquoise"
            | "violet"
            | "wheat"
            | "white"
            | "whitesmoke"
            | "yellow"
            | "yellowgreen"
    )
}

fn iter_function_args(inner: &str) -> impl Iterator<Item = &str> {
    // rgb()/hsl() arguments in the suite are comma-separated without nested functions.
    inner.split(',').map(str::trim).filter(|s| !s.is_empty())
}

pub(super) fn is_valid_rgb_like_function(
    token_lower: &str,
    has_alpha: bool,
    lenient: bool,
) -> bool {
    if lenient {
        // Modern CSS allows `rgb()`/`rgba()` with space syntax, relative colors, `none` components,
        // and nested functions. In lenient mode we only validate that it's a balanced call.
        return is_balanced_function_call_token(
            token_lower,
            if has_alpha { "rgba" } else { "rgb" },
        );
    }

    let name_len = if has_alpha { 5 } else { 4 }; // "rgba(" or "rgb("
    let Some(without_close) = token_lower.strip_suffix(")") else {
        return false;
    };
    if without_close.len() <= name_len {
        return false;
    }
    let inner = &without_close[name_len..];
    if inner.trim_start().starts_with("from ") {
        return is_balanced_function_call_token(
            token_lower,
            if has_alpha { "rgba" } else { "rgb" },
        );
    }
    if !inner.contains(',') {
        return is_valid_rgb_like_space_syntax(inner, has_alpha);
    }
    let mut args = iter_function_args(inner);
    let (Some(c1), Some(c2), Some(c3)) = (args.next(), args.next(), args.next()) else {
        return false;
    };
    let alpha = args.next();
    if args.next().is_some() {
        return false;
    }

    let is_percent = c1.ends_with('%') || c2.ends_with('%') || c3.ends_with('%');
    if is_percent {
        if !(c1.ends_with('%') && c2.ends_with('%') && c3.ends_with('%')) {
            return false;
        }
        if !is_percentage_0_100(c1) || !is_percentage_0_100(c2) || !is_percentage_0_100(c3) {
            return false;
        }
    } else if !is_integer_0_255(c1) || !is_integer_0_255(c2) || !is_integer_0_255(c3) {
        return false;
    }

    if has_alpha {
        alpha.is_some_and(is_alpha_value)
    } else {
        alpha.is_none()
    }
}

fn is_valid_rgb_like_space_syntax(inner: &str, has_alpha: bool) -> bool {
    let mut parts = inner.split('/');
    let before = parts.next().unwrap_or("");
    let after = parts.next();
    if parts.next().is_some() {
        return false;
    }

    let comps: Vec<&str> = before
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .collect();
    if comps.len() != 3 {
        return false;
    }

    let alpha = if let Some(s) = after {
        let tokens: Vec<&str> = s
            .split_ascii_whitespace()
            .filter(|t| !t.is_empty())
            .collect();
        if tokens.len() != 1 {
            return false;
        }
        Some(tokens[0])
    } else {
        None
    };

    let is_percent = comps.iter().any(|c| c.ends_with('%'));
    if is_percent {
        if !comps
            .iter()
            .all(|c| c.ends_with('%') && is_percentage_0_100(c))
        {
            return false;
        }
    } else if !comps.iter().all(|c| is_integer_0_255(c)) {
        return false;
    }

    if has_alpha {
        alpha.is_some_and(is_alpha_value)
    } else {
        alpha.is_none_or(is_alpha_value)
    }
}

pub(super) fn is_valid_hsl_like_function(
    token_lower: &str,
    has_alpha: bool,
    lenient: bool,
) -> bool {
    if lenient {
        // Modern CSS allows `hsl()`/`hsla()` with relative colors, `none` components, and nested
        // functions. In lenient mode we only validate that it's a balanced call.
        return is_balanced_function_call_token(
            token_lower,
            if has_alpha { "hsla" } else { "hsl" },
        );
    }

    let name_len = if has_alpha { 5 } else { 4 }; // "hsla(" or "hsl("
    let Some(without_close) = token_lower.strip_suffix(")") else {
        return false;
    };
    if without_close.len() <= name_len {
        return false;
    }
    let inner = &without_close[name_len..];
    if inner.trim_start().starts_with("from ") {
        return is_balanced_function_call_token(
            token_lower,
            if has_alpha { "hsla" } else { "hsl" },
        );
    }
    if !inner.contains(',') {
        return is_valid_hsl_like_space_syntax(inner, has_alpha);
    }
    let mut args = iter_function_args(inner);
    let (Some(_h), Some(s), Some(l)) = (args.next(), args.next(), args.next()) else {
        return false;
    };
    let alpha = args.next();
    if args.next().is_some() {
        return false;
    }

    // Keep this permissive for now: check only that saturation/lightness are percentages.
    if !(s.ends_with('%') && l.ends_with('%')) {
        return false;
    }
    if !is_percentage_0_100(s) || !is_percentage_0_100(l) {
        return false;
    }

    if has_alpha {
        alpha.is_some_and(is_alpha_value)
    } else {
        alpha.is_none()
    }
}

fn is_valid_hsl_like_space_syntax(inner: &str, has_alpha: bool) -> bool {
    let mut parts = inner.split('/');
    let before = parts.next().unwrap_or("");
    let after = parts.next();
    if parts.next().is_some() {
        return false;
    }

    let comps: Vec<&str> = before
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .collect();
    if comps.len() != 3 {
        return false;
    }

    let alpha = if let Some(s) = after {
        let tokens: Vec<&str> = s
            .split_ascii_whitespace()
            .filter(|t| !t.is_empty())
            .collect();
        if tokens.len() != 1 {
            return false;
        }
        Some(tokens[0])
    } else {
        None
    };

    let s = comps[1];
    let l = comps[2];
    if !(s.ends_with('%') && l.ends_with('%')) {
        return false;
    }
    if !is_percentage_0_100(s) || !is_percentage_0_100(l) {
        return false;
    }

    if has_alpha {
        alpha.is_some_and(is_alpha_value)
    } else {
        alpha.is_none_or(is_alpha_value)
    }
}

fn is_percentage_0_100(s: &str) -> bool {
    let Some(num) = s.strip_suffix('%') else {
        return false;
    };
    parse_css_number(num).is_some_and(|v| (0.0..=100.0).contains(&v))
}

fn is_integer_0_255(s: &str) -> bool {
    let t = s.trim();
    if t.is_empty() {
        return false;
    }
    if t.contains('.') {
        return false;
    }
    let Ok(v) = t.parse::<i32>() else {
        return false;
    };
    (0..=255).contains(&v)
}

fn is_alpha_value(s: &str) -> bool {
    let t = s.trim();
    if let Some(pct) = t.strip_suffix('%') {
        return parse_css_number(pct).is_some_and(|v| (0.0..=100.0).contains(&v));
    }
    parse_css_number(t).is_some_and(|v| (0.0..=1.0).contains(&v))
}
