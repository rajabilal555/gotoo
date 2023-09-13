use core::fmt;

pub enum Editor {
    VsCode,
    Neovim,
    Vim,
}
impl fmt::Display for Editor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.as_str())
    }
}

impl Editor {
    pub fn as_str(&self) -> &str {
        match self {
            Editor::VsCode => "vscode",
            Editor::Neovim => "nvim",
            Editor::Vim => "vim",
        }
    }
}
