use _macros;

struct VItem { 
  x: int 
}

// NB. We're using the 'a' lifetime to force the type interference on collect()
fn v1<'a> (a:~[&'a str]) -> ~[&'a str] {
  return a;
}

#[test]
fn test_can_create_struct() {
  let x = v1("Hello World".split(' ').collect());
}
