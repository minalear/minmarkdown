use regex::Regex;

#[derive(Debug)]
pub enum BlockTypes {
  Paragraph,
  Blockquote,
  Header(u32),
  OrderedList,
  UnorderedList,
  Code,
  HorizontalRule,
  Raw // Generic 


  // Table - not implemented
  // InlineHTML - not implemented
}

#[derive(Debug)]
pub struct Block {
  pub text: String,
  pub block_type: BlockTypes
}

impl Block {
  fn new(text: String, block_type: BlockTypes) -> Block {
    Block { text, block_type }
  }
}

pub fn parse(markdown: &str) -> String {
  let mut blocks = Vec::new();
  let mut buffer = String::new();

  // breakdown markdown into blocks of text
  for line in markdown.lines() {
    if line.len() == 0 && buffer.len() != 0 {
      blocks.push(parse_block(&buffer));
      buffer.clear();
    } else if line.len() != 0 {
      buffer.push_str(line);
      buffer.push_str("\n");
    }
  }

  // if our buffer isn't empty, push the contents into a new block
  if buffer.len() != 0 {
    blocks.push(Block::new(buffer.clone(), BlockTypes::Raw));
    buffer.clear();
  }
  let blocks = blocks;

  // generate HTML from blocks
  for block in blocks {
    match block.block_type {
      BlockTypes::Paragraph => buffer.push_str(&format!("<p>{}</p>\n", block.text)),
      BlockTypes::Blockquote => buffer.push_str(&format!("<blockquote>{}</blockquote>\n", block.text)),
      BlockTypes::OrderedList => buffer.push_str(&format!("<ol>{}</ol>\n", block.text)),
      BlockTypes::UnorderedList => buffer.push_str(&format!("<ul>{}</ul>\n", block.text)),
      BlockTypes::Code => buffer.push_str(&format!("<pre><code>{}</code></pre>\n", block.text)),
      _ => { 
        buffer.push_str(&block.text);
        buffer.push_str("\n");
      }
    }
  }

  buffer
}

fn parse_block(text: &str) -> Block {
  let mut block_type = BlockTypes::Raw;
  let mut buffer = String::with_capacity(text.len());
  
  // parse each line to determine block type and to apply formatting
  for line in text.lines() {
    match block_type {
      // Due to the unique nature of Code blocks, we have to treat them differently
      // and not apply formatting within the code block.
      BlockTypes::Code => {
        buffer.push_str(&line);
      },
      _ => {
        if line.begins_with("```") {
          // Code blocks are designated with ``` before and after
          // so we don't need to parse the rest of the line
          block_type = BlockTypes::Code;
          buffer.push_str("<pre><code>");
        } else if line.begins_with("# ") {
          block_type = BlockTypes::Header(1);
          let line = format!("<h1>{}</h1>", line.replace("# ", ""));
          buffer.push_str(&apply_formatting(&line));
        } else if line.begins_with("## ") {
          block_type = BlockTypes::Header(2);
          let line = format!("<h2>{}</h2>", line.replace("## ", ""));
          buffer.push_str(&apply_formatting(&line));
        } else if line.begins_with("### ") {
          block_type = BlockTypes::Header(3);
          let line = format!("<h3>{}</h3>", line.replace("### ", ""));
          buffer.push_str(&apply_formatting(&line));
        } else if line.begins_with("#### ") {
          block_type = BlockTypes::Header(4);
          let line = format!("<h4>{}</h4>", line.replace("#### ", ""));
          buffer.push_str(&apply_formatting(&line));
        } else if line.begins_with("##### ") {
          block_type = BlockTypes::Header(5);
          let line = format!("<h5>{}</h5>", line.replace("##### ", ""));
          buffer.push_str(&apply_formatting(&line));
        } else if line.begins_with("###### ") {
          block_type = BlockTypes::Header(6);
          let line = format!("<h6>{}</h6>", line.replace("###### ", ""));
          buffer.push_str(&apply_formatting(&line));
        } else if line.begins_with("> ") {
          block_type = BlockTypes::Blockquote;
          buffer.push_str(&apply_formatting(&line.replace("> ", "")));
        } else if line.begins_with("1. ") {
          // TODO: Implement (#). lists
          block_type = BlockTypes::OrderedList;
          buffer.push_str(&apply_formatting(&format!("<li>{}</li>", line)));
        } else if line.begins_with("* ") {
          block_type = BlockTypes::UnorderedList;
          buffer.push_str(&apply_formatting(&format!("<li>{}</li>", line)));
        } else if line.begins_with("---") || line.begins_with("***") || line.begins_with("___") { 
          block_type = BlockTypes::HorizontalRule;
          buffer.push_str("<hr>");
        } else {
          block_type = BlockTypes::Paragraph;
          buffer.push_str(&apply_formatting(&line));
        }
      }
    }
  }

  Block::new(buffer.clone(), block_type)
}

fn apply_formatting(line: &str) -> String {
  lazy_static! {
    static ref BOLD: Regex = Regex::new(r"\*\*(.*?)*\*\*").unwrap();
    static ref ITALIC: Regex = Regex::new(r"\*(.*?)*\*").unwrap();
    static ref STRIKE: Regex = Regex::new(r"~~(.*?)~~").unwrap();
    static ref URL: Regex = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
  }

  let line = BOLD.replace_all(line, "<strong>$1</strong>");
  let line = ITALIC.replace_all(&line, "<strong>$1</strong>");
  let line = STRIKE.replace_all(&line, "<strong>$1</strong>");
  let line = URL.replace_all(&line, "<strong>$1</strong>");
 
  String::from(line)
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
  fn parse_test() {
    let md = 
r"
# Header

## Subheader

This is a paragraph of some importance.
";

    let blocks = parse(&md);
    println!("{:?}", blocks);

    assert!(blocks.len() == 3);
  }
}