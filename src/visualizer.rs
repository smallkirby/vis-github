use crate::context::*;
use crate::analyzer::separator::*;

pub fn visualize_by_time(context: &Context, timemap: CommitTimeMap) {
  println!("{}", context.owner);
  println!("{:?}", timemap);
}