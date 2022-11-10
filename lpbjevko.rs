enum Mode {
  Len = 0,
  Prefix = 128,
  Suffix = 129,
  End = 130,
  Done = 131
}

trait JevkoParseStreamRecv {
  fn event(&mut self, mode: u8, buf: &Vec<u8>);
}

struct JevkoParseStream<'a> {
  next: &'a mut dyn JevkoParseStreamRecv,
  mode: u8,
  depth: u32,
  len: usize,
  buf: Vec<u8>,
}

const MAX_BYTES: usize = ((usize::BITS / 8) + 1) as usize;

trait JevkoParseStreamTrait {
  fn bytes(&mut self, bytes: &[u8]);
}

impl<'a> JevkoParseStreamTrait for JevkoParseStream<'a> {
  fn bytes(&mut self, bytes: &[u8]) {
    let mut i = 0;
    while i < bytes.len() {
      let b = bytes[i];
      if self.mode == Mode::Len as u8 {
        if b < 128 {
          if self.buf.len() < MAX_BYTES {
            self.buf.push(b);
          } else {
            panic!("1");
          }
        } else {
          for d in &self.buf {
            self.len = (self.len << 7) | (*d as usize);
          }
          self.buf = vec![];
          self.mode = b;
        }
      } else if self.mode <= Mode::End as u8 {
        if self.buf.len() < self.len {
          self.buf.push(b)
        } else {
          i -= 1;
          self.next.event(self.mode, &self.buf);
          self.buf = vec![];
          self.len = 0;
          if self.mode == Mode::Prefix as u8 {
            self.depth += 1;
            self.mode = Mode::Len as u8;
          } else if self.mode == Mode::Suffix as u8 {
            if self.depth == 0 {
              panic!("4");
            }
            self.depth -= 1;
            self.mode = Mode::Len as u8;
          } else if self.mode == Mode::End as u8 {
            if self.depth != 0 {
              panic!("3");
            }
            self.mode = Mode::Done as u8;
          }
        }
      } else {
        panic!("2");
      }
      i += 1;
    }
    if self.mode == Mode::End as u8 {
      let ret = self.next.event(self.mode, &self.buf);
      self.buf = vec![];
      return ret;
    }
  }
}

fn make_parser(x: &mut dyn JevkoParseStreamRecv) -> JevkoParseStream {
  return JevkoParseStream {
    next: x,
    mode: Mode::Len as u8,
    depth: 0,
    len: 0,
    buf: vec![],
  };
}

fn main() {
  struct T {}
  impl JevkoParseStreamRecv for T {
    fn event(&mut self, mode: u8, buf: &Vec<u8>) {
      println!("{}, {:?}", mode, buf);
    }
  }
  let mut u = T {};

  let mut st = make_parser(&mut u);

  let testbytes: &[u8] = &[
    5, Mode::Prefix as u8, 0, 0, 0, 0, 0,
    3, Mode::Suffix as u8, 0, 0, 0,
  
    Mode::Prefix as u8, 3, Mode::Suffix as u8, 1,1,1,
    Mode::End as u8
  ];

  st.bytes(&testbytes);
}