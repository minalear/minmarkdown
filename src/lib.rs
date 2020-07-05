use regex::Regex;

enum GroupedTypes {
  Paragraph,
  Quote,
  Code
}

pub fn to_html(markdown: &str) -> String {
  let bold_re = Regex::new(r"\*\*(.*?)*\*\*").unwrap();
  let italic_re = Regex::new(r"\*(.*?)*\*").unwrap();
  let strike_re = Regex::new(r"~~(.*?)~~").unwrap();
  let url_re = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();

  // parse markdown input line by line
  let mut html = String::new();
  let mut grouped = String::new();
  let mut group_type = GroupedTypes::Paragraph;

  for line in markdown.lines() {
    match group_type {
      GroupedTypes::Code => {
        if line.begins_with("```") {
          group_type = GroupedTypes::Paragraph;
          html.push_str(&format!("<pre>{}</pre>", grouped));
          grouped.clear();
        } else {
          grouped.push_str(line);
          grouped.push_str("\n");
        }
      },
      _ => {
        // blank lines signify end of paragraphs
        if line.len() == 0 {
          if grouped.len() != 0 {
            let format = match group_type {
              GroupedTypes::Paragraph => {
                format!("<p>{}</p>\n", grouped) 
            },
              GroupedTypes::Quote => format!("<blockquote><p>{}</p></blockquote>", grouped),
              _ => String::from("")
            };

            html.push_str(&format);
            grouped.clear();  
          }
        } else {
          let line = bold_re.replace_all(line, "<strong>$1</strong>");
          let line = italic_re.replace_all(&line, "<em>$1</em>");
          let line = strike_re.replace_all(&line, "<s>$1</s>");
          let line = url_re.replace_all(&line, "<a href=\"$2\">$1</a>");

          let line = line.trim();
          
          // MD Tags
          if line.begins_with("```") { // code section
            group_type = GroupedTypes::Code;
          }
          else if line.begins_with("# ") { // H1 header
            html.push_str(&format!("<h1>{}</h1>\n", line.replace("# ", "")));
          } else if line.begins_with("## ") { // H2 header
            html.push_str(&format!("<h2>{}</h2>\n", line.replace("## ", "")));
          } else if line.begins_with("### ") { // H3 header
            html.push_str(&format!("<h3>{}</h3>\n", line.replace("### ", "")));
          } else if line.begins_with("#### ") { // H4 header
            html.push_str(&format!("<h4>{}</h4>\n", line.replace("#### ", "")));
          } else if line.begins_with("##### ") { // H5 header
            html.push_str(&format!("<h5>{}</h5>\n", line.replace("##### ", "")));
          } else if line.begins_with("###### ") { // H6 header
            html.push_str(&format!("<h6>{}</h6>\n", line.replace("###### ", "")));
          } else if line.begins_with("> ") { // Blockquotes
            group_type = GroupedTypes::Quote;
            grouped.push_str(&line.replace("> ", "")); // building a blockquote
            grouped.push_str("\n");
          } else { // Basic paragraph
            group_type = GroupedTypes::Paragraph;
            grouped.push_str(line); // building a paragraph
            grouped.push_str("\n");
          }
        }
      }
    }
  }

  html
}

// Extend string to have begins_with function
trait BeginsWithExt {
  fn begins_with(&self, pattern: &str) -> bool;
}
impl BeginsWithExt for String {
  fn begins_with(&self, pattern: &str) -> bool {
    match pattern.len() > self.len() {
      true => false,
      false => &self[0..pattern.len()] == pattern
    }
  }
}
impl BeginsWithExt for &str {
  fn begins_with(&self, pattern: &str) -> bool {
    match pattern.len() > self.len() {
      true => false,
      false => &self[0..pattern.len()] == pattern
    }
  }
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

  #[test]
  fn begins_with() {
    let a = String::from("> This is a blockquote.");
    let b = String::from(">> This is not a blockquote.");
    let pattern = "> ";

    assert!(a.begins_with(pattern));
    assert!(!b.begins_with(pattern));
  }
}