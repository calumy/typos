use structopt::StructOpt;

use crate::config;

arg_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Format {
        Silent,
        Brief,
        Long,
        Json,
    }
}

pub const PRINT_SILENT: typos_cli::report::PrintSilent = typos_cli::report::PrintSilent;
pub const PRINT_BRIEF: typos_cli::report::PrintBrief = typos_cli::report::PrintBrief;
pub const PRINT_LONG: typos_cli::report::PrintLong = typos_cli::report::PrintLong;
pub const PRINT_JSON: typos_cli::report::PrintJson = typos_cli::report::PrintJson;

impl Format {
    pub(crate) fn reporter(self) -> &'static dyn typos_cli::report::Report {
        match self {
            Format::Silent => &PRINT_SILENT,
            Format::Brief => &PRINT_BRIEF,
            Format::Long => &PRINT_LONG,
            Format::Json => &PRINT_JSON,
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::Long
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
#[structopt(
        setting = structopt::clap::AppSettings::UnifiedHelpMessage,
        setting = structopt::clap::AppSettings::DeriveDisplayOrder,
        setting = structopt::clap::AppSettings::DontCollapseArgsInUsage
    )]
#[structopt(group = structopt::clap::ArgGroup::with_name("mode").multiple(false))]
pub(crate) struct Args {
    #[structopt(parse(from_os_str), default_value = ".")]
    /// Paths to check with `-` for stdin
    pub(crate) path: Vec<std::path::PathBuf>,

    #[structopt(short = "c", long = "config")]
    /// Custom config file
    pub(crate) custom_config: Option<std::path::PathBuf>,

    #[structopt(long)]
    /// Ignore implicit configuration files.
    pub(crate) isolated: bool,

    #[structopt(long, group = "mode")]
    /// Print a diff of what would change
    pub(crate) diff: bool,

    #[structopt(long, short = "w", group = "mode")]
    /// Write fixes out
    pub(crate) write_changes: bool,

    #[structopt(long, group = "mode")]
    /// Debug: Print each file that would be spellchecked.
    pub(crate) files: bool,

    #[structopt(long, group = "mode")]
    /// Debug: Print each identifier that would be spellchecked.
    pub(crate) identifiers: bool,

    #[structopt(long, group = "mode")]
    /// Debug: Print each word that would be spellchecked.
    pub(crate) words: bool,

    #[structopt(long, group = "mode")]
    /// Write the current configuration to file with `-` for stdout
    pub(crate) dump_config: Option<std::path::PathBuf>,

    #[structopt(flatten)]
    pub(crate) overrides: FileArgs,

    #[structopt(
        long,
        possible_values(&Format::variants()),
        case_insensitive(true),
        default_value("long")
    )]
    pub(crate) format: Format,

    #[structopt(short = "j", long = "threads", default_value = "0")]
    /// The approximate number of threads to use.
    pub(crate) threads: usize,

    #[structopt(flatten)]
    pub(crate) config: ConfigArgs,

    #[structopt(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(crate) struct FileArgs {
    #[structopt(long, overrides_with("no-binary"))]
    /// Search binary files.
    binary: bool,
    #[structopt(long, overrides_with("binary"), hidden(true))]
    no_binary: bool,

    #[structopt(long, overrides_with("check-filenames"))]
    /// Skip verifying spelling in file names.
    no_check_filenames: bool,
    #[structopt(long, overrides_with("no-check-filenames"), hidden(true))]
    check_filenames: bool,

    #[structopt(long, overrides_with("check-files"))]
    /// Skip verifying spelling in filess.
    no_check_files: bool,
    #[structopt(long, overrides_with("no-check-files"), hidden(true))]
    check_files: bool,

    #[structopt(
        long,
        possible_values(&config::Locale::variants()),
    )]
    pub(crate) locale: Option<config::Locale>,
}

impl config::EngineSource for FileArgs {
    fn binary(&self) -> Option<bool> {
        match (self.binary, self.no_binary) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn check_filename(&self) -> Option<bool> {
        match (self.check_filenames, self.no_check_filenames) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn check_file(&self) -> Option<bool> {
        match (self.check_files, self.no_check_files) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn dict(&self) -> Option<&dyn config::DictSource> {
        Some(self)
    }
}

impl config::DictSource for FileArgs {
    fn locale(&self) -> Option<config::Locale> {
        self.locale
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(crate) struct ConfigArgs {
    #[structopt(flatten)]
    walk: WalkArgs,
}

impl config::ConfigSource for ConfigArgs {
    fn walk(&self) -> Option<&dyn config::WalkSource> {
        Some(&self.walk)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub(crate) struct WalkArgs {
    #[structopt(long, overrides_with("no-hidden"))]
    /// Search hidden files and directories.
    hidden: bool,
    #[structopt(long, overrides_with("hidden"), hidden(true))]
    no_hidden: bool,

    #[structopt(long, overrides_with("ignore"))]
    /// Don't respect ignore files.
    no_ignore: bool,
    #[structopt(long, overrides_with("no-ignore"), hidden(true))]
    ignore: bool,

    #[structopt(long, overrides_with("ignore-dot"))]
    /// Don't respect .ignore files.
    no_ignore_dot: bool,
    #[structopt(long, overrides_with("no-ignore-dot"), hidden(true))]
    ignore_dot: bool,

    #[structopt(long, overrides_with("ignore-global"))]
    /// Don't respect global ignore files.
    no_ignore_global: bool,
    #[structopt(long, overrides_with("no-ignore-global"), hidden(true))]
    ignore_global: bool,

    #[structopt(long, overrides_with("ignore-parent"))]
    /// Don't respect ignore files in parent directories.
    no_ignore_parent: bool,
    #[structopt(long, overrides_with("no-ignore-parent"), hidden(true))]
    ignore_parent: bool,

    #[structopt(long, overrides_with("ignore-vcs"))]
    /// Don't respect ignore files in vcs directories.
    no_ignore_vcs: bool,
    #[structopt(long, overrides_with("no-ignore-vcs"), hidden(true))]
    ignore_vcs: bool,
}

impl config::WalkSource for WalkArgs {
    fn ignore_hidden(&self) -> Option<bool> {
        match (self.hidden, self.no_hidden) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn ignore_files(&self) -> Option<bool> {
        match (self.no_ignore, self.ignore) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn ignore_dot(&self) -> Option<bool> {
        match (self.no_ignore_dot, self.ignore_dot) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn ignore_vcs(&self) -> Option<bool> {
        match (self.no_ignore_vcs, self.ignore_vcs) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn ignore_global(&self) -> Option<bool> {
        match (self.no_ignore_global, self.ignore_global) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }

    fn ignore_parent(&self) -> Option<bool> {
        match (self.no_ignore_parent, self.ignore_parent) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (_, _) => unreachable!("StructOpt should make this impossible"),
        }
    }
}
