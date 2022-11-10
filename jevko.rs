use std::iter::FromIterator;

trait JevkoParseStreamRecv {
  fn prefix(&mut self, text: String);
  fn suffix(&mut self, text: String);
  fn end(&mut self, text: String);
}

struct JevkoParseStream<'a> {
  next: &'a mut dyn JevkoParseStreamRecv,
  is_escaped: bool,
  depth: i64,
  text: String,
  opener: char, 
  closer: char, 
  escaper: char,
}

trait JevkoParseStreamTrait {
  fn chunk(&mut self, ch: String);
  fn end(&mut self);
}

impl<'a> JevkoParseStreamTrait for JevkoParseStream<'a> {
  fn end(&mut self) {
    if self.is_escaped {
      panic!("Unexpected end after escaper ({})!", self.escaper)
    }
    if self.depth != 0 {
      panic!("Unexpected end: missing {} closer(s) ({})!", self.depth, self.closer)
    }
    // if (parents.length !== 1) {
    //   const parentInfo = parents.pop()
    //   // todo: say which ln, col unclosed
    //   throw SyntaxError(`Unexpected end: missing ${parents.length} closer(s) (${closer})!`)
    // }
    self.next.end(self.text.clone());
    // escapedAt = []
    self.text = String::new();
    // todo: maybe reset all state or forbid calling chunk again; self.chunk = () => throw Error
    // return ret
  }
  fn chunk(&mut self, ch: String) {
    let mut h = 0;
    let mut i = 0;
    let len = ch.chars().count();

    while i < len {
      match ch.chars().nth(i) {
        Some(code) =>
          if self.is_escaped {
            if code == self.escaper || code == self.opener || code == self.closer {
              self.is_escaped = false;
            } else {
              // todo: line, col instead of 0,0
              panic!("Invalid digraph ({}) at {}:{}!", String::from_iter([self.escaper, code]), 0,0)
            }
          } else if code == self.escaper {
            self.is_escaped = true;
            match ch.get(h..i) {
              Some(substr) => self.text += substr,
              None => panic!()
            }
            
            // escapedAt.push(textBuffer.length)
            h = i + 1;
          } else if code == self.opener {
            // if (parents.length >= maxDepth) throw Error(`Invalid parser state! Max depth of ${maxDepth} exceeded!`)

            self.depth += 1;
            match ch.get(h..i) {
              Some(substr) => {
                self.next.prefix(self.text.clone() + substr);
                self.text = String::new();
              },
              None => panic!()
            }
            
            // escapedAt = []
            h = i + 1;
          } else if code == self.closer {
            // if (parents.length === 1) throw SyntaxError(`Unexpected closer (${closer}) at ${line}:${column}!`)
            self.depth -= 1;
            match ch.get(h..i) {
              Some(substr) => {
                self.next.suffix(self.text.clone() + substr);
                self.text = String::new();
              },
              None => panic!()
            }
            // escapedAt = []
            h = i + 1;
          }
        None => panic!()
      }

      i += 1;
    }
    match ch.get(h..i) {
      Some(substr) => {
        self.text += substr;
      },
      None => panic!()
    }
  }
}

fn make_parser(x: &mut dyn JevkoParseStreamRecv) -> JevkoParseStream {
  return JevkoParseStream {
    next: x,
    is_escaped: false,
    depth: 0,
    text: String::new(),
    opener: '[',
    closer: ']',
    escaper: '`',
  };
}

fn main() {
  struct T {}
  impl JevkoParseStreamRecv for T {
    fn prefix(&mut self, text: String) {
      println!("{}", text);
    }
    fn suffix(&mut self, text: String) {
      println!("{}", text);
    }
    fn end(&mut self, text: String) {
      println!("{}", text);
    }
  }
  let mut u = T {};

  let mut st = make_parser(&mut u);

  st.chunk("a [b] c".to_string());
  st.end();
}