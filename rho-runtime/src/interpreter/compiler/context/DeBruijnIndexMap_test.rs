
#[test]
fn test_index_map() {
    // Suppose we have following rholang code
    // new a1, a2, a3 in {
    //     new b1, b2, a3 in {
    //         Nil
    //     }
    // }
    let mut level1 = DeBruijnIndexMap::empty().create(vec![
        ("a1".to_string(), VarSort::Name, SourcePosition{ row : 1, column : 2 } ),
        ("a2".to_string(), VarSort::Name, SourcePosition{ row : 1, column : 5 } ),
        ("a3".to_string(), VarSort::Name, SourcePosition{ row : 1, column : 8 } ),
    ]);
    let level2 = level1.create(vec![
        ("b1".to_string(), VarSort::Name, SourcePosition{ row : 2, column : 3 } ),
        ("b2".to_string(), VarSort::Name, SourcePosition{ row : 2, column : 6 } ),
        ("a3".to_string(), VarSort::Name, SourcePosition{ row : 2, column : 9 } ),
    ]);

    match level1.get("a1") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 2),
    };
    match level1.get("a2") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 1),
    };
    match level1.get("a3") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 0),
    };
    assert!(level1.get("b1").is_none());
    assert!(level1.get("b2").is_none());

    match level2.get("a1") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 5),
    };
    match level2.get("a2") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 4),
    };
    match level2.get("b1") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 2),
    };
    match level2.get("b2") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 1),
    };
    match level2.get("a3") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 0),
    };
}