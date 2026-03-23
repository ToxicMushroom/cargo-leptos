use clap::{ArgGroup, Args};

use crate::internal_prelude::*;

// A subset of the cargo-generate commands available.
// See: https://github.com/cargo-generate/cargo-generate/blob/main/src/args.rs

#[derive(Clone, Debug, Args, PartialEq, Eq)]
#[clap(arg_required_else_help(true))]
#[clap(group(ArgGroup::new("template").args(&["git", "path"]).required(true).multiple(false)))]
#[clap(about)]
pub struct NewCommand {
    /// Git repository to clone template from. Can be a full URL (like
    /// `https://github.com/leptos-rs/start-actix`), or a shortcut for one of our
    /// built-in templates. Recommended shortcuts are:
    ///
    /// `leptos-rs/start-actix`, `leptos-rs/start-axum`,
    /// `leptos-rs/start-axum-workspace`, `leptos-rs/start-csr`.
    #[clap(short, long, group = "git-arg")]
    pub git: Option<String>,

    /// Branch to use when installing from git
    #[clap(short, long, conflicts_with = "tag", requires = "git-arg")]
    pub branch: Option<String>,

    /// Tag to use when installing from git
    #[clap(short, long, conflicts_with = "branch", requires = "git-arg")]
    pub tag: Option<String>,

    /// Local path to copy the template from. Can not be specified together with --git.
    #[clap(short, long)]
    pub path: Option<String>,

    /// Directory to create / project name; if the name isn't in kebab-case, it will be converted
    /// to kebab-case unless `--force` is given.
    #[clap(long, short, value_parser)]
    pub name: Option<String>,

    /// Don't convert the project name to kebab-case before creating the directory.
    /// Note that cargo generate won't overwrite an existing directory, even if `--force` is given.
    #[clap(long, short, action)]
    pub force: bool,

    /// Enables more verbose output.
    #[clap(long, short, action)]
    pub verbose: bool,

    /// Generate the template directly into the current dir. No subfolder will be created and no vcs is initialized.
    #[clap(long, action)]
    pub init: bool,
}

impl NewCommand {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}