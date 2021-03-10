mod bnfc;

fn main() {
    let s = bnfc::parse("new xx in { xx!(1) | Nil | 0 }");
    println!("{:?}", s);
}
