//! The five AppKit control sizes (`NSControl.ControlSize`).

/// The five AppKit control sizes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MacControlSize {
    ExtraLarge,
    Large,
    #[default]
    Regular,
    Small,
    Mini,
}

impl MacControlSize {
    /// The size name as used in macvue design tokens.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExtraLarge => "extra-large",
            Self::Large => "large",
            Self::Regular => "regular",
            Self::Small => "small",
            Self::Mini => "mini",
        }
    }
}

impl std::fmt::Display for MacControlSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
