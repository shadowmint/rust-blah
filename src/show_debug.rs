use std::fmt;

struct BlahLF {
    id: usize
}

impl fmt::Debug for BlahLF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hi: {}", self.id)
    }
}

#[test]
fn test_show() {
    let foo = BlahLF { id: 0usize };
    println!("{:?}", foo);
}
