use regex::Regex;

pub fn to_html(markdown: &str) -> String {
  let bold_re = Regex::new(r"\*\*(.*?)*\*\*").unwrap();
  let italic_re = Regex::new(r"\*(.*?)*\*").unwrap();
  let strike_re = Regex::new(r"~~(.*?)~~").unwrap();
  let url_re = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();

  // parse markdown input line by line
  let mut html = String::new();
  let mut paragraph = String::new();

  for line in markdown.lines() {
    // blank lines signify end of paragraphs
    if line.len() == 0 && paragraph.len() != 0 {
      html.push_str(&format!("<p>{}</p>\n", paragraph));
      paragraph.clear();  
    }
    
    let line = bold_re.replace_all(line, "<strong>$1</strong>");
    let line = italic_re.replace_all(&line, "<em>$1</em>");
    let line = strike_re.replace_all(&line, "<s>$1</s>");
    let line = url_re.replace_all(&line, "<a href=\"$2\">$1</a>");

    let line = line.trim();
    
    // Header Tags
    if line.contains("###### ") {
      html.push_str(&format!("<h6>{}</h6>\n", line.replace("###### ", "")));
    } else if line.contains("##### ") {
      html.push_str(&format!("<h5>{}</h5>\n", line.replace("##### ", "")));
    } else if line.contains("#### ") {
      html.push_str(&format!("<h4>{}</h4>\n", line.replace("#### ", "")));
    } else if line.contains("### ") {
      html.push_str(&format!("<h3>{}</h3>\n", line.replace("### ", "")));
    } else if line.contains("## ") {
      html.push_str(&format!("<h2>{}</h2>\n", line.replace("## ", "")));
    } else if line.contains("# ") {
      html.push_str(&format!("<h1>{}</h1>\n", line.replace("# ", "")));
    } else {
      paragraph.push_str(line); // building paragraphs
    }
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