use crate::{
  analyzer::separator::*,
  context::*,
  github::{language::*, license::*},
};

use console::{style, Emoji};

static EMOJI_GRASS: Emoji<'_, '_> = Emoji("🔍", "");
static EMOJI_CLOCK: Emoji<'_, '_> = Emoji("🕐", "");
static EMOJI_SCHOLAR: Emoji<'_, '_> = Emoji("🎓️", "");
static EMOJI_ABC: Emoji<'_, '_> = Emoji("🔤", "");

pub fn visualize_by_time(context: &Context, timemap: CommitTimeMap) {
  let total = timemap.iter().fold(0, |acc, (_hour, count)| acc + count);
  println!("");
  println!(
    "{} {}'s time map {}\n",
    EMOJI_GRASS,
    style(context.owner.clone()).green(),
    EMOJI_CLOCK
  );
  println!("  total: {} commits", style(total).yellow());
  println!("");

  let max_width = 100;
  for hour in 0..24 {
    let num = match timemap.get(&hour) {
      Some(count) => *count,
      None => 0,
    };
    let percentage = num as f64 / total as f64;
    let blocks = "▇".repeat((percentage * max_width as f64) as usize);
    println!(" {:>2} ({:>5.1}%) {}", hour, percentage * 100.0, blocks);
  }
}

pub fn visualize_by_license(context: &Context, license_map: LicenseMap) {
  let mut license_vec: Vec<(License, u64)> = license_map.into_iter().collect();
  license_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

  let total_repo = license_vec
    .iter()
    .fold(0, |acc, (_license, count)| acc + count);
  println!("");
  println!(
    "{} {}'s LICENSE map {}\n",
    EMOJI_GRASS,
    style(context.owner.clone()).green(),
    EMOJI_SCHOLAR
  );
  println!("  total: {} repos", style(total_repo).yellow());
  println!("");

  let max_width = 40;
  let max_name_width = license_vec.iter().fold(0, |acc, (license, _count)| {
    std::cmp::max(license.name.len(), acc)
  });

  for (license, count) in license_vec {
    let percentage = count as f64 / total_repo as f64;
    let blocks = "▇".repeat((percentage * max_width as f64) as usize);
    let space_num = max_name_width - license.name.len();
    print!(" {}", " ".repeat(space_num));
    println!(
      " {} ({:>5.1}%): {}",
      license.name,
      percentage * 100.0,
      blocks
    );
  }
}

pub fn visualize_by_language(context: &Context, language_map: &mut Vec<Language>) {
  language_map.sort_by(|a, b| b.lines.partial_cmp(&a.lines).unwrap());

  let total_langs_num = language_map.len();
  let total_lines = language_map.iter().fold(0, |acc, lang| lang.lines + acc);
  println!("");
  println!(
    "{} {}'s LANGUAGE map {}\n",
    EMOJI_GRASS,
    style(context.owner.clone()).green(),
    EMOJI_ABC
  );
  println!(
    "  total: {} langs ({} lines)",
    style(total_langs_num).yellow(),
    style(total_lines).blue()
  );
  println!("");

  let max_width = 40;
  let max_lines = language_map
    .iter()
    .fold(0, |m, lang| std::cmp::max(lang.lines, m));
  let max_block_length = ((max_lines as f64 / total_lines as f64) * max_width as f64) as usize;
  let max_name_width = language_map
    .iter()
    .fold(0, |acc, lang| std::cmp::max(lang.name.len(), acc));

  for language in language_map {
    let percentage = language.lines as f64 / total_lines as f64;
    let blocks = "▇".repeat((percentage * max_width as f64) as usize);
    let space_num = max_name_width - language.name.len();
    print!(" {}", " ".repeat(space_num));
    println!(
      " {} ({:>5.1}%): {} {} ({})",
      language.name,
      percentage * 100.0,
      blocks,
      " ".repeat(max_block_length - blocks.chars().count()),
      language.lines_readable(),
    );
  }
}
