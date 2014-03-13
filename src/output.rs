use std::io::stdio::stdout;

fn trace(x:int) {
  let mut out = stdout();
  out.write_le_int(x);
}

#[test]
fn test_trace_works() {
  trace(65);
  trace(65);
}
