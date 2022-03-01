use stable_eyre::eyre::*;
use chrono::prelude::*;

// Demonstratest how rust date parser deals with century for two digit dates.

fn test_date(date: &str) -> Result<()> {
  let d  = NaiveDate::parse_from_str(date, "%m/%d/%y") 
                 .with_context(|| format!("Error when parsing date: {}", date.to_owned()))?;
  println!("{} -> {}", date, d.format("%Y-%m-%d").to_string());
  Ok(())
}

fn main() -> Result<()> {
    test_date("01/01/69")?;
    test_date("01/01/71")?;
    Ok(())
}

