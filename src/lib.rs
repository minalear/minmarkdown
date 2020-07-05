use regex::Regex;

pub fn to_html(markdown: &str) -> String {
  let bold_re = Regex::new(r"\*\*(.*?)*\*\*").unwrap();
  let italic_re = Regex::new(r"\*(.*?)*\*").unwrap();
  let strike_re = Regex::new(r"~~(.*?)~~").unwrap();

  let mut html = String::new();
  for line in markdown.lines() {
    // ignore blank lines
    if line.len() == 0 { continue; }
    
    let line = bold_re.replace_all(line, "<strong>$1</strong>");
    let line = italic_re.replace_all(&line, "<em>$1</em>");
    let line = strike_re.replace_all(&line, "<s>$1</s>");

    html.push_str(&line);
  }

  html
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn md_to_html() {
    let md = "This **is** a test *line* that should ~~have~~ some **stuff**.";
    let html = to_html(md);

    println!("{}", html);
  }
}