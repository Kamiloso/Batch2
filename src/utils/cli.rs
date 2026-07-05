#![allow(unused_imports)]

macro_rules! _cli_print {
    ($($arg:tt)*) => {{
        std::print!($($arg)*);
        std::io::Write::flush(&mut std::io::stdout())
            .expect("unable to flush stdout");
    }};
}

macro_rules! _cli_println {
    ($($arg:tt)*) => {
        std::println!($($arg)*);
    };
}

macro_rules! _cli_eprint {
    ($($arg:tt)*) => {{
        std::eprint!($($arg)*);
        std::io::Write::flush(&mut std::io::stderr())
            .expect("unable to flush stderr");
    }};
}

macro_rules! _cli_eprintln {
    ($($arg:tt)*) => {
        std::eprintln!($($arg)*);
    };
}

macro_rules! _cli_input {
    ($($arg:tt)*) => {{
        $crate::utils::cli::print!($($arg)*);

        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("unable to read from stdin");

        let len = line.trim_end_matches(['\r', '\n']).len();
        line.truncate(len);
        
        line
    }};
}

macro_rules! _cli_multiline_input {
    ($($arg:tt)*) => {{
        $crate::utils::cli::print!($($arg)*);

        let mut result = String::new();
        
        // TODO: double ^^ must convert into ^ and odd amount indicates newline (only at the end)

        loop {
            let mut line = String::new();
            std::io::stdin()
                .read_line(&mut line)
                .expect("unable to read from stdin");

            let clean_line = line.trim_end_matches(['\r', '\n']);

            if clean_line.ends_with("^^") {
                result.push_str(&clean_line[..clean_line.len() - 1]);
                break;
            } else if clean_line.ends_with('^') {
                result.push_str(&clean_line[..clean_line.len() - 1]);
                result.push('\n');
                $crate::utils::cli::print!("More? ");
            } else {
                result.push_str(clean_line);
                break;
            }
        }

        result
    }};
}

macro_rules! _cli_clear {
    () => {{
        clearscreen::clear()
            .expect("failed to clear terminal");
    }};
}

pub(crate) use _cli_print as print;
pub(crate) use _cli_println as println;
pub(crate) use _cli_eprint as eprint;
pub(crate) use _cli_eprintln as eprintln;
pub(crate) use _cli_input as input;
pub(crate) use _cli_multiline_input as multiline_input;
pub(crate) use _cli_clear as clear;