macro_rules! _regex_matches {
    ($value:expr, $re:literal) => {{
        static RE: std::sync::OnceLock<::regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| ::regex::Regex::new($re).expect("invalid regex pattern"))
            .is_match($value)
    }};
}

macro_rules! _regex_captures {
    ($value:expr, $re:literal) => {{
        static RE: std::sync::OnceLock<::regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| ::regex::Regex::new($re).expect("invalid regex pattern"))
            .captures($value)
    }};
}

pub(crate) use _regex_matches as matches;
pub(crate) use _regex_captures as captures;