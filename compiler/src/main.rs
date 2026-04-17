//! `aura` — the AURA toolchain CLI entry point.
//!
//! Parses command-line arguments and dispatches to the appropriate
//! handler in `compiler::cmd`.

use clap::{Parser, Subcommand};
use compiler::error::Result;
use std::path::PathBuf;

// -------------------------------------------------------------------- //
// CLI definitions (clap derive)

#[derive(Parser)]
#[command(
  name = "aura",
  about = "AURA toolchain — compile, validate, and manage .aura projects"
)]
#[command(version = "0.3.0-alpha.2")]
struct Cli {
  #[command(subcommand)]
  command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
  /// Compile .aura source files to .atom / .hami / .atlas artifacts.
  Compile {
    /// Project root directory (default: current directory).
    #[arg(long)]
    project: Option<PathBuf>,
    /// Compile a historical take by ID instead of the working draft.
    #[arg(long)]
    take: Option<String>,
    /// Embed HistoryNodes (0x14) into the .atom output.
    #[arg(long)]
    embed_history: bool,
    /// Output directory (default: dist/).
    #[arg(long)]
    out: Option<PathBuf>,
    /// Treat unresolved references as errors.
    #[arg(long)]
    strict: bool,
  },
  /// Validate syntax and reference resolution without emitting output.
  Validate {
    #[arg(long)]
    project: Option<PathBuf>,
    #[arg(long)]
    strict: bool,
  },
  /// Check for style violations and best-practice warnings.
  Lint {
    #[arg(long)]
    project: Option<PathBuf>,
    /// Enable W006: warn on keys outside the standard AURA vocabulary.
    #[arg(long)]
    strict: bool,
  },
  /// Generate a new typed ID.
  Generate {
    /// Type name, e.g. track, person, episode, collection.
    type_name: String,
  },
  /// Scaffold a new AURA project.
  Init {
    /// Media kind, e.g. audio::album, video::movie.
    kind: String,
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    lang: Option<String>,
    #[arg(long)]
    dir: Option<PathBuf>,
  },
  /// Add a new content file to the current project.
  Add {
    /// Content type, e.g. track, episode, scene.
    type_name: String,
    /// Human-readable label for the new file.
    label: String,
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// Record the current working draft as an immutable take.
  Take {
    /// Optional descriptive message.
    message: Option<String>,
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// Attach a human-readable name to the current take.
  Mark {
    name: String,
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// Restore the draft to a previous take (non-destructive).
  Rewind {
    /// Take ID, mark name, or ~N (relative).
    target: String,
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// Show the full take history.
  Ledger {
    /// Optional node path to filter history.
    node_path: Option<String>,
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// Show changed nodes between two takes.
  Delta {
    take_a: String,
    take_b: String,
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// Manage named development streams.
  Stream {
    #[command(subcommand)]
    action: StreamCmd,
  },
  /// Mix a completed stream into the current stream.
  Mix {
    stream: String,
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// Park the current working draft without recording a take.
  Hold {
    /// Restore a previously parked draft.
    #[arg(long)]
    restore: bool,
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// Publish the current take to the cloud store.
  Release {
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// Pull the latest released state from the cloud store.
  Sync {
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// Create an independent full-history copy of the project.
  Dub {
    destination: PathBuf,
    #[arg(long)]
    project: Option<PathBuf>,
  },
}

#[derive(Subcommand)]
enum StreamCmd {
  /// Open a new named development stream.
  Open {
    name: String,
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// Close and archive a named stream.
  Close {
    name: String,
    #[arg(long)]
    project: Option<PathBuf>,
  },
  /// List all open streams.
  List {
    #[arg(long)]
    project: Option<PathBuf>,
  },
}

// -------------------------------------------------------------------- //
// Entry point

fn main() -> Result<()> {
  let cli = Cli::parse();
  let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

  use compiler::cmd;

  match cli.command {
    Cmd::Compile {
      project,
      take,
      embed_history,
      out,
      strict,
    } => cmd::compile::run(&cmd::compile::CompileOpts {
      project: project.unwrap_or_else(|| cwd.clone()),
      take,
      embed_history,
      out_dir: out,
      strict,
    }),
    Cmd::Validate { project, strict } => cmd::compile::validate(&project.unwrap_or(cwd), strict),
    Cmd::Lint { project, strict } => cmd::compile::lint(&project.unwrap_or(cwd), strict),
    Cmd::Generate { type_name } => cmd::gen::run(&type_name),
    Cmd::Init {
      kind,
      name,
      lang,
      dir,
    } => cmd::init::init(&cmd::init::InitOpts {
      kind,
      name,
      lang,
      dir,
    }),
    Cmd::Add {
      type_name,
      label,
      project,
    } => cmd::init::add(&type_name, &label, &project.unwrap_or(cwd)),
    Cmd::Take { message, project } => cmd::take::take(&project.unwrap_or(cwd), message.as_deref()),
    Cmd::Mark { name, project } => cmd::take::mark(&project.unwrap_or(cwd), &name),
    Cmd::Rewind { target, project } => cmd::take::rewind(&project.unwrap_or(cwd), &target),
    Cmd::Ledger { node_path, project } => {
      cmd::take::ledger(&project.unwrap_or(cwd), node_path.as_deref())
    }
    Cmd::Delta {
      take_a,
      take_b,
      project,
    } => cmd::take::delta(&project.unwrap_or(cwd), &take_a, &take_b),
    Cmd::Stream { action } => match action {
      StreamCmd::Open { name, project } => cmd::stream::open(&project.unwrap_or(cwd), &name),
      StreamCmd::Close { name, project } => cmd::stream::close(&project.unwrap_or(cwd), &name),
      StreamCmd::List { project } => cmd::stream::list(&project.unwrap_or(cwd)),
    },
    Cmd::Mix { stream, project } => cmd::stream::mix(&project.unwrap_or(cwd), &stream),
    Cmd::Hold { restore, project } => {
      let proj = &project.unwrap_or(cwd);
      if restore {
        cmd::hold::restore(proj)
      } else {
        cmd::hold::hold(proj)
      }
    }
    Cmd::Release { project } => cmd::cloud::release(&project.unwrap_or(cwd)),
    Cmd::Sync { project } => cmd::cloud::sync(&project.unwrap_or(cwd)),
    Cmd::Dub {
      destination,
      project,
    } => cmd::cloud::dub(&project.unwrap_or(cwd), &destination),
  }
}
