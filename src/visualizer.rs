use crate::context::*;
use crate::analyzer::separator::*;

pub fn visualize_by_time(context: &Context, timemap: CommitTimeMap) {
  let total = timemap.iter().fold(0, |acc, (_hour, count)| acc + count);
  println!("{}'s time map", context.owner);
  println!("  total: {} commits", total);
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
