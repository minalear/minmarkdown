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

pub fn parse(markdown: &str) -> Vec<Block> {
  // first split the markdown into blocks typed Raw
  let mut blocks = Vec::new();
  let mut buffer = String::new();
  for line in markdown.lines() {
    if line.len() == 0 && buffer.len() != 0 {
      blocks.push(Block::new(buffer.clone(), BlockTypes::Raw));
      buffer.clear();
    } else if line.len() != 0 {
      buffer.push_str(line);
    }
  }

  // if our buffer isn't empty, push the contents into a new block
  if buffer.len() != 0 {
    blocks.push(Block::new(buffer.clone(), BlockTypes::Raw));
  }

  blocks
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