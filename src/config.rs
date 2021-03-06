use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct LinesConfig {
  /// Whether lines only consisting of a comment should be excluded
  #[structopt(short = "ic", long)]
  pub include_comments: bool,

  /// Whether empty lines should be included
  #[structopt(short = "ie", long)]
  pub include_empty: bool,
}
