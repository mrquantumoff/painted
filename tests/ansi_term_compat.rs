#![cfg(not(feature = "no-color"))]
#![allow(unused_imports)]

extern crate ansi_term;
extern crate painted;

use ansi_term::*;
use painted::*;

macro_rules! test_simple_color {
    ($string:expr, $painted_name:ident, $ansi_term_name:ident) => {
        #[test]
        fn $painted_name() {
            let s = format!("{} {}", $string, stringify!($painted_name));
            assert_eq!(
                s.$painted_name().to_string(),
                Colour::$ansi_term_name.paint(s).to_string()
            )
        }
    };
}

mod compat_colors {
    use super::ansi_term::*;
    use super::painted::*;

    test_simple_color!("test string", black, Black);
    test_simple_color!("test string", red, Red);
    test_simple_color!("test string", green, Green);
    test_simple_color!("test string", yellow, Yellow);
    test_simple_color!("test string", blue, Blue);
    test_simple_color!("test string", magenta, Purple);
    test_simple_color!("test string", cyan, Cyan);
    test_simple_color!("test string", white, White);
}

macro_rules! test_simple_style {
    ($string:expr, $style:ident) => {
        #[test]
        fn $style() {
            let s = format!("{} {}", $string, stringify!($style));
            assert_eq!(
                s.$style().to_string(),
                ansi_term::Style::new().$style().paint(s).to_string()
            )
        }
    };
}

mod compat_styles {
    use super::ansi_term;
    use super::ansi_term::*;
    use super::painted;
    use super::painted::*;

    test_simple_style!("test string", bold);
    test_simple_style!("test string", dimmed);
    test_simple_style!("test string", italic);
    test_simple_style!("test string", underline);
    test_simple_style!("test string", blink);
    test_simple_style!("test string", reverse);
    test_simple_style!("test string", hidden);
}

macro_rules! test_simple_bgcolor {
    ($string:expr, $painted_name:ident, $ansi_term_name:ident) => {
        #[test]
        fn $painted_name() {
            let s = format!("{} {}", $string, stringify!($painted_name));
            assert_eq!(
                s.$painted_name().to_string(),
                ansi_term::Style::default()
                    .on(ansi_term::Colour::$ansi_term_name)
                    .paint(s)
                    .to_string()
            )
        }
    };
}

mod compat_bgcolors {
    use super::ansi_term;
    use super::ansi_term::*;
    use super::painted;
    use super::painted::*;

    test_simple_bgcolor!("test string", on_black, Black);
    test_simple_bgcolor!("test string", on_red, Red);
    test_simple_bgcolor!("test string", on_green, Green);
    test_simple_bgcolor!("test string", on_yellow, Yellow);
    test_simple_bgcolor!("test string", on_blue, Blue);
    test_simple_bgcolor!("test string", on_magenta, Purple);
    test_simple_bgcolor!("test string", on_cyan, Cyan);
    test_simple_bgcolor!("test string", on_white, White);
}

mod compat_complex {
    use super::ansi_term;
    use super::ansi_term::*;
    use super::painted;
    use super::painted::*;

    #[test]
    fn complex1() {
        let s = "test string";
        let ansi = Colour::Red.on(Colour::Black).bold().italic().paint(s);
        assert_eq!(
            ansi.to_string(),
            s.red().bold().italic().on_black().to_string()
        );
    }

    #[test]
    fn complex2() {
        let s = "test string";
        let ansi = Colour::Green.on(Colour::Yellow).underline().paint(s);
        assert_eq!(
            ansi.to_string(),
            s.green().on_yellow().underline().to_string()
        );
    }
}

mod compat_overrides {
    use super::ansi_term;
    use super::ansi_term::*;
    use super::painted;
    use super::painted::*;

    #[test]
    fn overrides1() {
        let s = "test string";
        let ansi = Colour::Red.on(Colour::Black).on(Colour::Blue).paint(s);
        assert_eq!(ansi.to_string(), s.red().on_blue().to_string());
    }

    #[test]
    fn overrides2() {
        let s = "test string";
        let ansi = Colour::Green.on(Colour::Yellow).paint(s);
        assert_eq!(
            ansi.to_string(),
            s.green().on_yellow().green().on_yellow().to_string()
        );
    }
}
