
#[test]
fn test_index_map() {
    // Suppose we have following rholang code
    // new a1, a2, a3 in {
    //     new b1, b2, a3 in {
    //         Nil
    //     }
    // }
    let mut level1 = DeBruijnIndexMap::empty().clone_then_put(vec![
        ("a1".to_string(), VarSort::Name, SourcePosition{ row : 1, column : 2, length : 0 } ),
        ("a2".to_string(), VarSort::Name, SourcePosition{ row : 1, column : 5, length : 0 } ),
        ("a3".to_string(), VarSort::Name, SourcePosition{ row : 1, column : 8, length : 0 } ),
    ]);
    let level2 = level1.clone_then_put(vec![
        ("b1".to_string(), VarSort::Name, SourcePosition{ row : 2, column : 3, length : 0 } ),
        ("b2".to_string(), VarSort::Name, SourcePosition{ row : 2, column : 6, length : 0 } ),
        ("a3".to_string(), VarSort::Name, SourcePosition{ row : 2, column : 9, length : 0 } ),
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