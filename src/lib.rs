use zed_extension_api as zed;

struct Codeowners;

impl zed::Extension for Codeowners {
    fn new() -> Self {
        Codeowners
    }
}

zed::register_extension!(Codeowners);
