


mod psend_tests {
    use model::*;
    use super::*;
    use super::super::*;



   #[test]
   fn test() {
    let rholang_code = "new x in {
        x!(1)
    }";

    let proc = builder::parse(rholang_code).unwrap();
   }


}