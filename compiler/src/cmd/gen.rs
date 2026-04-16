//! `aura generate {type}` — ID generation command.

use crate::error::{CompileError, Result};
use aura::id::{IdGen, Prefix};

/// Generates a new unique ID for the given type name.
///
/// Prints the ID to stdout so it can be captured by the caller.
///
/// # Examples
///
/// ```sh
/// aura generate track      # → t7xab3c
/// aura generate person     # → p4xt9k2
/// aura generate episode    # → ep7xb3n
/// aura generate collection # → c8xab3d
/// ```
pub fn run(type_name: &str) -> Result<()> {
  let prefix = type_name_to_prefix(type_name)?;
  let mut gen = IdGen::new();
  let id = gen.generate(prefix);
  println!("{}", id);
  Ok(())
}

fn type_name_to_prefix(name: &str) -> Result<Prefix> {
  match name {
    "track" => Ok(Prefix::Track),
    "collection" => Ok(Prefix::Collection),
    "person" => Ok(Prefix::Person),
    "annotator" => Ok(Prefix::Person), // annotators use p prefix
    "variant" => Ok(Prefix::Variant),
    "episode" => Ok(Prefix::Episode),
    "season" => Ok(Prefix::Season),
    "series" => Ok(Prefix::Series),
    "film" => Ok(Prefix::Film),
    "documentary" => Ok(Prefix::Documentary),
    "podcast" => Ok(Prefix::Podcast),
    "animation" => Ok(Prefix::Animation),
    "speech" => Ok(Prefix::Speech),
    "book" => Ok(Prefix::Book),
    "music-video" => Ok(Prefix::MusicVideo),
    "single" => Ok(Prefix::Single),
    "interview" => Ok(Prefix::Interview),
    "rights" => Ok(Prefix::Rights),
    "take" => Ok(Prefix::Take),
    "studio" => Ok(Prefix::Studio),
    "label" => Ok(Prefix::Label),
    "art" => Ok(Prefix::Art),
    "motion" => Ok(Prefix::Motion),
    "trailer" => Ok(Prefix::Trailer),
    _ => Err(CompileError::msg(format!(
      "unknown type `{}`. Valid types: track, collection, person, \
             annotator, variant, episode, season, series, film, documentary, \
             podcast, animation, speech, book, music-video, single, interview, \
             rights, take, studio, label, art, motion, trailer",
      name
    ))),
  }
}
