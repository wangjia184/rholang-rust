mod bnfc;

fn main() {
    let s = bnfc::parse("new xx in { xx!(1) }");
    println!("{:?}", s);
}
