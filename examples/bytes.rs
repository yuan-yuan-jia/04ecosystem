use anyhow::Result;
use bytes::{BytesMut, BufMut};

fn main() -> Result<()> {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(b"hello world\n");
    buf.put(&b"goodbye world"[..]);
    buf.put_i64(0xdeadbeef);

    println!("{:?}", buf);
    let a = buf.split();
    let mut b = a.freeze();

    let c = b.split_to(12);
    println!("c={:?}", c);
    println!("b={:?}", b);
    println!("buf={:?}", buf);
    Ok(())
}