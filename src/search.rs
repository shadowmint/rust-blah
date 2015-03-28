use std::str::from_utf8;
use std::raw::Slice as Slice_;
use std::mem::transmute;

#[test]
fn test_seek() {
  let mut x:Vec<u8> = Vec::new();
  x.push_all("Hello World Adfadf oi 英字新聞のジャパンタイムズがおくる通訳・翻訳業界総合 dfs dsdsfsdf ffガイド2015年度版が登場！a adf adsf das  dasf das fzc xv cvzx cxzv czvx czxv df adf adf".as_bytes());
  x.push_all(&[255u8, 255u8, 255u8, 255u8, 0u8]); // Invalid utf8 data
  println!("Boundary: {:?}", find_utf_boundary(x.as_slice()));
}

fn find_utf_boundary(value:&[u8]) -> isize {
  unsafe {
    let tmp:Slice_<u8> = transmute(value);
    let mut maybe_invalid = 0is;
    let mut marker = 0is;
    for i in 0..tmp.len {
      let view:Slice_<u8> = Slice_ { data:tmp.data, len: i };
      let view_as_bytes:&[u8] = transmute(view);
      if from_utf8(view_as_bytes).is_err() {
        if maybe_invalid == 0 {
          marker = i as isize;
          maybe_invalid = 1;
        }
        else {
          maybe_invalid += 1;
        }
        println!("Found {:?} consequetive invalid bytes", maybe_invalid);
        if maybe_invalid == 4 { // see http://stackoverflow.com/questions/4983196/unicode-code-point-limit
          return marker;
        }
      }
      else {
        println!("Found valid utf8 data; resetting counter");
        maybe_invalid = 0;
      }
    }
  }
  return -1;
}
