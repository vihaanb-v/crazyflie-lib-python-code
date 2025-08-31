use std::fmt::Display;

pub(crate) struct Error {
    msg: String,
    debug: Option<String>,
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self {
            msg: value.to_string(),
            debug: None,
        }
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self { msg, debug: None }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error {
    pub(crate) fn with_debug<S: AsRef<str>>(mut self, new_d: S) -> Self {
        let new_d = new_d.as_ref().trim();
        if new_d.is_empty() {
            return self;
        }
        let new_debug = match self.debug {
            None => new_d.into(),
            Some(mut debug) => {
                debug.push('\n');
                debug.push_str(new_d);
                debug
            }
        };
        self.debug = Some(new_debug);
        self
    }

    pub(crate) fn debug_info(&self) -> Option<&str> {
        self.debug.as_deref()
    }
}
