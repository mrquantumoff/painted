//!Coloring terminal so simple, you already know how to do it !
//!
//!    use painted::Colorize;
//!
//!    "this is blue".blue();
//!    "this is red".red();
//!    "this is red on blue".red().on_blue();
//!    "this is also red on blue".on_blue().red();
//!    "you can use truecolor values too!".truecolor(0, 255, 136);
//!    "background truecolor also works :)".on_truecolor(135, 28, 167);
//!    "you can also make bold comments".bold();
//!    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
//!    "or change advice. This is red".yellow().blue().red();
//!    "or clear things up. This is default color and style".red().bold().clear();
//!    "purple and magenta are the same".purple().magenta();
//!    "bright colors are also allowed".bright_blue().on_bright_white();
//!    "you can specify color by string".color("blue").on_color("red");
//!    "and so are normal and clear".normal().clear();
//!    String::from("this also works!").green().bold();
//!    format!("{:30}", "format works as expected. This will be padded".blue());
//!    format!("{:.3}", "and this will be green but truncated to 3 chars".green());
//!
//!
//! See [the `Colorize` trait](./trait.Colorize.html) for all the methods.
//!
#![warn(missing_docs)]

extern crate atty;
#[macro_use]
extern crate lazy_static;
#[cfg(windows)]
extern crate winapi;

#[cfg(test)]
extern crate rspec;

mod color;
pub mod control;
mod style;

pub use self::customcolors::CustomColor;

/// Custom colors support.
pub mod customcolors;

pub use color::*;

use std::{borrow::Cow, fmt, ops::Deref};

pub use style::{Style, Styles};

/// A string that may have color and/or style applied to it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct paintedString {
    input: String,
    fgcolor: Option<Color>,
    bgcolor: Option<Color>,
    style: style::Style,
}

/// The trait that enables something to be given color.
///
/// You can use `painted` effectively simply by importing this trait
/// and then using its methods on `String` and `&str`.
#[allow(missing_docs)]
pub trait Colorize {
    // Font Colors
    fn black(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::Black)
    }
    fn red(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::Red)
    }
    fn green(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::Green)
    }
    fn yellow(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::Yellow)
    }
    fn blue(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::Blue)
    }
    fn magenta(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::Magenta)
    }
    fn purple(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::Magenta)
    }
    fn cyan(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::Cyan)
    }
    fn white(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::White)
    }
    fn bright_black(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::BrightBlack)
    }
    fn bright_red(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::BrightRed)
    }
    fn bright_green(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::BrightGreen)
    }
    fn bright_yellow(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::BrightYellow)
    }
    fn bright_blue(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::BrightBlue)
    }
    fn bright_magenta(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::BrightMagenta)
    }
    fn bright_purple(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::BrightMagenta)
    }
    fn bright_cyan(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::BrightCyan)
    }
    fn bright_white(self) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::BrightWhite)
    }
    fn truecolor(self, r: u8, g: u8, b: u8) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::TrueColor { r, g, b })
    }
    fn custom_color(self, color: CustomColor) -> paintedString
    where
        Self: Sized,
    {
        self.color(Color::TrueColor {
            r: color.r,
            g: color.g,
            b: color.b,
        })
    }
    fn color<S: Into<Color>>(self, color: S) -> paintedString;
    // Background Colors
    fn on_black(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::Black)
    }
    fn on_red(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::Red)
    }
    fn on_green(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::Green)
    }
    fn on_yellow(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::Yellow)
    }
    fn on_blue(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::Blue)
    }
    fn on_magenta(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::Magenta)
    }
    fn on_purple(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::Magenta)
    }
    fn on_cyan(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::Cyan)
    }
    fn on_white(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::White)
    }
    fn on_bright_black(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::BrightBlack)
    }
    fn on_bright_red(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::BrightRed)
    }
    fn on_bright_green(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::BrightGreen)
    }
    fn on_bright_yellow(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::BrightYellow)
    }
    fn on_bright_blue(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::BrightBlue)
    }
    fn on_bright_magenta(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::BrightMagenta)
    }
    fn on_bright_purple(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::BrightMagenta)
    }
    fn on_bright_cyan(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::BrightCyan)
    }
    fn on_bright_white(self) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::BrightWhite)
    }
    fn on_truecolor(self, r: u8, g: u8, b: u8) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::TrueColor { r, g, b })
    }
    fn on_custom_color(self, color: CustomColor) -> paintedString
    where
        Self: Sized,
    {
        self.on_color(Color::TrueColor {
            r: color.r,
            g: color.g,
            b: color.b,
        })
    }
    fn on_color<S: Into<Color>>(self, color: S) -> paintedString;
    // Styles
    fn clear(self) -> paintedString;
    fn normal(self) -> paintedString;
    fn bold(self) -> paintedString;
    fn dimmed(self) -> paintedString;
    fn italic(self) -> paintedString;
    fn underline(self) -> paintedString;
    fn blink(self) -> paintedString;
    /// Historical name of `Colorize::reversed`. May be removed in a future version. Please use
    /// `Colorize::reversed` instead
    fn reverse(self) -> paintedString;
    /// This should be preferred to `Colorize::reverse`.
    fn reversed(self) -> paintedString;
    fn hidden(self) -> paintedString;
    fn strikethrough(self) -> paintedString;
}

impl paintedString {
    /// Get the current background color applied.
    ///
    /// ```rust
    /// # use painted::*;
    /// let cstr = "".blue();
    /// assert_eq!(cstr.fgcolor(), Some(Color::Blue));
    /// let cstr = cstr.clear();
    /// assert_eq!(cstr.fgcolor(), None);
    /// ```
    pub fn fgcolor(&self) -> Option<Color> {
        self.fgcolor.as_ref().copied()
    }

    /// Get the current background color applied.
    ///
    /// ```rust
    /// # use painted::*;
    /// let cstr = "".on_blue();
    /// assert_eq!(cstr.bgcolor(), Some(Color::Blue));
    /// let cstr = cstr.clear();
    /// assert_eq!(cstr.bgcolor(), None);
    /// ```
    pub fn bgcolor(&self) -> Option<Color> {
        self.bgcolor.as_ref().copied()
    }

    /// Get the current [`Style`] which can be check if it contains a [`Styles`].
    ///
    /// ```rust
    /// # use painted::*;
    /// let painted = "".bold().italic();
    /// assert_eq!(painted.style().contains(Styles::Bold), true);
    /// assert_eq!(painted.style().contains(Styles::Italic), true);
    /// assert_eq!(painted.style().contains(Styles::Dimmed), false);
    /// ```
    pub fn style(&self) -> style::Style {
        self.style
    }

    /// Checks if the painted string has no color or styling.
    ///
    /// ```rust
    /// # use painted::*;
    /// let cstr = "".red();
    /// assert_eq!(cstr.is_plain(), false);
    /// let cstr = cstr.clear();
    /// assert_eq!(cstr.is_plain(), true);
    /// ```
    pub fn is_plain(&self) -> bool {
        self.bgcolor.is_none() && self.fgcolor.is_none() && self.style == style::CLEAR
    }

    #[cfg(not(feature = "no-color"))]
    fn has_colors(&self) -> bool {
        control::SHOULD_COLORIZE.should_colorize()
    }

    #[cfg(feature = "no-color")]
    fn has_colors(&self) -> bool {
        false
    }

    fn compute_style(&self) -> String {
        if !self.has_colors() || self.is_plain() {
            return String::new();
        }

        let mut res = String::from("\x1B[");
        let mut has_wrote = if self.style != style::CLEAR {
            res.push_str(&self.style.to_str());
            true
        } else {
            false
        };

        if let Some(ref bgcolor) = self.bgcolor {
            if has_wrote {
                res.push(';');
            }

            res.push_str(&bgcolor.to_bg_str());
            has_wrote = true;
        }

        if let Some(ref fgcolor) = self.fgcolor {
            if has_wrote {
                res.push(';');
            }

            res.push_str(&fgcolor.to_fg_str());
        }

        res.push('m');
        res
    }

    fn escape_inner_reset_sequences(&self) -> Cow<str> {
        if !self.has_colors() || self.is_plain() {
            return self.input.as_str().into();
        }

        // TODO: BoyScoutRule
        let reset = "\x1B[0m";
        let style = self.compute_style();
        let matches: Vec<usize> = self
            .input
            .match_indices(reset)
            .map(|(idx, _)| idx)
            .collect();
        if matches.is_empty() {
            return self.input.as_str().into();
        }

        let mut input = self.input.clone();
        input.reserve(matches.len() * style.len());

        for (idx_in_matches, offset) in matches.into_iter().enumerate() {
            // shift the offset to the end of the reset sequence and take in account
            // the number of matches we have escaped (which shift the index to insert)
            let mut offset = offset + reset.len() + idx_in_matches * style.len();

            for cchar in style.chars() {
                input.insert(offset, cchar);
                offset += 1;
            }
        }

        input.into()
    }
}

impl Default for paintedString {
    fn default() -> Self {
        paintedString {
            input: String::default(),
            fgcolor: None,
            bgcolor: None,
            style: style::CLEAR,
        }
    }
}

impl Deref for paintedString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.input
    }
}

impl<'a> From<&'a str> for paintedString {
    fn from(s: &'a str) -> Self {
        paintedString {
            input: String::from(s),
            ..paintedString::default()
        }
    }
}

impl Colorize for paintedString {
    fn color<S: Into<Color>>(mut self, color: S) -> paintedString {
        self.fgcolor = Some(color.into());
        self
    }
    fn on_color<S: Into<Color>>(mut self, color: S) -> paintedString {
        self.bgcolor = Some(color.into());
        self
    }

    fn clear(self) -> paintedString {
        paintedString {
            input: self.input,
            ..paintedString::default()
        }
    }
    fn normal(self) -> paintedString {
        self.clear()
    }
    fn bold(mut self) -> paintedString {
        self.style.add(style::Styles::Bold);
        self
    }
    fn dimmed(mut self) -> paintedString {
        self.style.add(style::Styles::Dimmed);
        self
    }
    fn italic(mut self) -> paintedString {
        self.style.add(style::Styles::Italic);
        self
    }
    fn underline(mut self) -> paintedString {
        self.style.add(style::Styles::Underline);
        self
    }
    fn blink(mut self) -> paintedString {
        self.style.add(style::Styles::Blink);
        self
    }
    fn reverse(self) -> paintedString {
        self.reversed()
    }
    fn reversed(mut self) -> paintedString {
        self.style.add(style::Styles::Reversed);
        self
    }
    fn hidden(mut self) -> paintedString {
        self.style.add(style::Styles::Hidden);
        self
    }
    fn strikethrough(mut self) -> paintedString {
        self.style.add(style::Styles::Strikethrough);
        self
    }
}

impl<'a> Colorize for &'a str {
    fn color<S: Into<Color>>(self, color: S) -> paintedString {
        paintedString {
            fgcolor: Some(color.into()),
            input: String::from(self),
            ..paintedString::default()
        }
    }

    fn on_color<S: Into<Color>>(self, color: S) -> paintedString {
        paintedString {
            bgcolor: Some(color.into()),
            input: String::from(self),
            ..paintedString::default()
        }
    }

    fn clear(self) -> paintedString {
        paintedString {
            input: String::from(self),
            style: style::CLEAR,
            ..paintedString::default()
        }
    }
    fn normal(self) -> paintedString {
        self.clear()
    }
    fn bold(self) -> paintedString {
        paintedString::from(self).bold()
    }
    fn dimmed(self) -> paintedString {
        paintedString::from(self).dimmed()
    }
    fn italic(self) -> paintedString {
        paintedString::from(self).italic()
    }
    fn underline(self) -> paintedString {
        paintedString::from(self).underline()
    }
    fn blink(self) -> paintedString {
        paintedString::from(self).blink()
    }
    fn reverse(self) -> paintedString {
        self.reversed()
    }
    fn reversed(self) -> paintedString {
        paintedString::from(self).reversed()
    }
    fn hidden(self) -> paintedString {
        paintedString::from(self).hidden()
    }
    fn strikethrough(self) -> paintedString {
        paintedString::from(self).strikethrough()
    }
}

impl fmt::Display for paintedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.has_colors() || self.is_plain() {
            return <String as fmt::Display>::fmt(&self.input, f);
        }

        // XXX: see tests. Useful when nesting painted strings
        let escaped_input = self.escape_inner_reset_sequences();

        f.write_str(&self.compute_style())?;
        escaped_input.fmt(f)?;
        f.write_str("\x1B[0m")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting() {
        // respect the formatting. Escape sequence add some padding so >= 40
        assert!(format!("{:40}", "".blue()).len() >= 40);
        // both should be truncated to 1 char before coloring
        assert_eq!(
            format!("{:1.1}", "toto".blue()).len(),
            format!("{:1.1}", "1".blue()).len()
        )
    }

    #[test]
    fn it_works() {
        let toto = "toto";
        println!("{}", toto.red());
        println!("{}", String::from(toto).red());
        println!("{}", toto.blue());

        println!("blue style ****");
        println!("{}", toto.bold());
        println!("{}", "yeah ! Red bold !".red().bold());
        println!("{}", "yeah ! Yellow bold !".bold().yellow());
        println!("{}", toto.bold().blue());
        println!("{}", toto.blue().bold());
        println!("{}", toto.blue().bold().underline());
        println!("{}", toto.blue().italic());
        println!("******");
        println!("test clearing");
        println!("{}", "red cleared".red().clear());
        println!("{}", "bold cyan cleared".bold().cyan().clear());
        println!("******");
        println!("Bg tests");
        println!("{}", toto.green().on_blue());
        println!("{}", toto.on_magenta().yellow());
        println!("{}", toto.purple().on_yellow());
        println!("{}", toto.magenta().on_white());
        println!("{}", toto.cyan().on_green());
        println!("{}", toto.black().on_white());
        println!("******");
        println!("{}", toto.green());
        println!("{}", toto.yellow());
        println!("{}", toto.purple());
        println!("{}", toto.magenta());
        println!("{}", toto.cyan());
        println!("{}", toto.white());
        println!("{}", toto.white().red().blue().green());
        println!("{}", toto.truecolor(255, 0, 0));
        println!("{}", toto.truecolor(255, 255, 0));
        println!("{}", toto.on_truecolor(0, 80, 80));
        // uncomment to see term output
        // assert!(false)
    }

    #[test]
    fn compute_style_empty_string() {
        assert_eq!("", "".clear().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_simple_fg_blue() {
        let blue = "\x1B[34m";

        assert_eq!(blue, "".blue().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_simple_bg_blue() {
        let on_blue = "\x1B[44m";

        assert_eq!(on_blue, "".on_blue().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_blue_on_blue() {
        let blue_on_blue = "\x1B[44;34m";

        assert_eq!(blue_on_blue, "".blue().on_blue().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_simple_fg_bright_blue() {
        let blue = "\x1B[94m";

        assert_eq!(blue, "".bright_blue().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_simple_bg_bright_blue() {
        let on_blue = "\x1B[104m";

        assert_eq!(on_blue, "".on_bright_blue().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_bright_blue_on_bright_blue() {
        let blue_on_blue = "\x1B[104;94m";

        assert_eq!(
            blue_on_blue,
            "".bright_blue().on_bright_blue().compute_style()
        );
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_simple_bold() {
        let bold = "\x1B[1m";

        assert_eq!(bold, "".bold().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_blue_bold() {
        let blue_bold = "\x1B[1;34m";

        assert_eq!(blue_bold, "".blue().bold().compute_style());
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn compute_style_blue_bold_on_blue() {
        let blue_bold_on_blue = "\x1B[1;44;34m";

        assert_eq!(
            blue_bold_on_blue,
            "".blue().bold().on_blue().compute_style()
        );
    }

    #[test]
    fn escape_reset_sequence_spec_should_do_nothing_on_empty_strings() {
        let style = paintedString::default();
        let expected = String::new();

        let output = style.escape_inner_reset_sequences();

        assert_eq!(expected, output);
    }

    #[test]
    fn escape_reset_sequence_spec_should_do_nothing_on_string_with_no_reset() {
        let style = paintedString {
            input: String::from("hello world !"),
            ..paintedString::default()
        };

        let expected = String::from("hello world !");
        let output = style.escape_inner_reset_sequences();

        assert_eq!(expected, output);
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn escape_reset_sequence_spec_should_replace_inner_reset_sequence_with_current_style() {
        let input = format!("start {} end", String::from("hello world !").red());
        let style = input.blue();

        let output = style.escape_inner_reset_sequences();
        let blue = "\x1B[34m";
        let red = "\x1B[31m";
        let reset = "\x1B[0m";
        let expected = format!("start {}hello world !{}{} end", red, reset, blue);
        assert_eq!(expected, output);
    }

    #[cfg_attr(feature = "no-color", ignore)]
    #[test]
    fn escape_reset_sequence_spec_should_replace_multiple_inner_reset_sequences_with_current_style()
    {
        let italic_str = String::from("yo").italic();
        let input = format!(
            "start 1:{} 2:{} 3:{} end",
            italic_str, italic_str, italic_str
        );
        let style = input.blue();

        let output = style.escape_inner_reset_sequences();
        let blue = "\x1B[34m";
        let italic = "\x1B[3m";
        let reset = "\x1B[0m";
        let expected = format!(
            "start 1:{}yo{}{} 2:{}yo{}{} 3:{}yo{}{} end",
            italic, reset, blue, italic, reset, blue, italic, reset, blue
        );

        println!("first: {}\nsecond: {}", expected, output);

        assert_eq!(expected, output);
    }

    #[test]
    fn color_fn() {
        assert_eq!("blue".blue(), "blue".color("blue"))
    }

    #[test]
    fn on_color_fn() {
        assert_eq!("blue".on_blue(), "blue".on_color("blue"))
    }

    #[test]
    fn bright_color_fn() {
        assert_eq!("blue".bright_blue(), "blue".color("bright blue"))
    }

    #[test]
    fn on_bright_color_fn() {
        assert_eq!("blue".on_bright_blue(), "blue".on_color("bright blue"))
    }

    #[test]
    fn exposing_tests() {
        let cstring = "".red();
        assert_eq!(cstring.fgcolor(), Some(Color::Red));
        assert_eq!(cstring.bgcolor(), None);

        let cstring = cstring.clear();
        assert_eq!(cstring.fgcolor(), None);
        assert_eq!(cstring.bgcolor(), None);

        let cstring = cstring.blue().on_bright_yellow();
        assert_eq!(cstring.fgcolor(), Some(Color::Blue));
        assert_eq!(cstring.bgcolor(), Some(Color::BrightYellow));

        let cstring = cstring.bold().italic();
        assert_eq!(cstring.fgcolor(), Some(Color::Blue));
        assert_eq!(cstring.bgcolor(), Some(Color::BrightYellow));
        assert_eq!(cstring.style().contains(Styles::Bold), true);
        assert_eq!(cstring.style().contains(Styles::Italic), true);
        assert_eq!(cstring.style().contains(Styles::Dimmed), false);
    }
}
