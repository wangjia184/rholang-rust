use super::*;

#[cfg(test)] 
fn traverse(depth : usize, iter : Box<dyn Iterator<Item = Node<'_>> + '_>) {
    for node in iter {
        match node {
            Node::Leaf(scored_atom) => {
                match scored_atom {
                    ScoreAtom::IntAtom(i) => println!("{:indent$}ScoreAtom::IntAtom({})", "", i, indent=depth*2),
                    ScoreAtom::StringAtom(s) => println!("{:indent$}ScoreAtom::StringAtom({})", "", s, indent=depth*2),
                    _ => { panic!("Unknown type") },
                }
                
            },
            Node::Children(iter) => {
                println!("{:indent$}{{", "", indent=depth*2);
                traverse(depth+1, iter);
                println!("{:indent$}}}", "", indent=depth*2);
            }
        }
        
    }
}




#[test]
fn news_should_be_sorted_basing_on_bind_count_urls_and_then_body() {
    let mut par = Par::default();
    {
        let mut rho_new = New::default();
        rho_new.bind_count = 2;
        rho_new.p = Some(Par::default());
        par.news.push(rho_new);
    }
    {
        let mut rho_new = New::default();
        rho_new.bind_count = 2;
        rho_new.uri.push("rho:io:stderr".to_owned());
        rho_new.p = Some(Par::default());
        par.news.push(rho_new);
    }
    {
        let mut rho_new = New::default();
        rho_new.bind_count = 2;
        rho_new.uri.push("rho:io:stdout".to_owned());
        rho_new.p = Some(Par::default());
        par.news.push(rho_new);
    }
    {
        let mut rho_new = New::default();
        rho_new.bind_count = 1;
        rho_new.p = Some(Par::default());
        par.news.push(rho_new);
    }
    {
        let mut rho_new = New::default();
        rho_new.bind_count = 2;
        rho_new.uri.push("rho:io:stdout".to_owned());
        let mut sub_par = Par::default();
        sub_par.exprs.push(
            Expr {
                expr_instance : Some(expr::ExprInstance::GInt(7))
            }
        );
        rho_new.p = Some(sub_par);
        par.news.push(rho_new);
    }

    // Node(List(
    //     Leaf(ScoreAtom(IntAtom(999))),
    //     Node(List(
    //         Leaf(ScoreAtom(IntAtom(303))),
    //         Leaf(ScoreAtom(IntAtom(1))),
    //         Leaf(ScoreAtom(IntAtom(0))),
    //         Leaf(ScoreAtom(IntAtom(0))),
    //         Node(List(
    //             Leaf(ScoreAtom(IntAtom(999))),
    //             Leaf(ScoreAtom(IntAtom(0)))
    //         ))
    //     )),
    //     Node(List(
    //         Leaf(ScoreAtom(IntAtom(303))),
    //         Leaf(ScoreAtom(IntAtom(2))),
    //         Leaf(ScoreAtom(IntAtom(0))),
    //         Leaf(ScoreAtom(IntAtom(0))),
    //         Node(List(
    //             Leaf(ScoreAtom(IntAtom(999))),
    //             Leaf(ScoreAtom(IntAtom(0)))
    //         ))
    //     )),
    //     Node(Vector(
    //         Leaf(ScoreAtom(IntAtom(303))),
    //         Leaf(ScoreAtom(IntAtom(2))),
    //         Leaf(ScoreAtom(StringAtom(rho:io:stderr))),
    //         Leaf(ScoreAtom(IntAtom(0))),
    //         Node(List(
    //             Leaf(ScoreAtom(IntAtom(999))),
    //             Leaf(ScoreAtom(IntAtom(0)))
    //         ))
    //     )), 
    //     Node(Vector(
    //         Leaf(ScoreAtom(IntAtom(303))),
    //         Leaf(ScoreAtom(IntAtom(2))),
    //         Leaf(ScoreAtom(StringAtom(rho:io:stdout))),
    //         Leaf(ScoreAtom(IntAtom(0))), 
    //         Node(List(
    //             Leaf(ScoreAtom(IntAtom(999))),
    //             Leaf(ScoreAtom(IntAtom(0)))
    //         ))
    //     )),
    //     Node(Vector(
    //         Leaf(ScoreAtom(IntAtom(303))),
    //         Leaf(ScoreAtom(IntAtom(2))),
    //         Leaf(ScoreAtom(StringAtom(rho:io:stdout))),
    //         Leaf(ScoreAtom(IntAtom(0))),
    //         Node(List(
    //             Leaf(ScoreAtom(IntAtom(999))),
    //             Node(ArrayBuffer(
    //                 Leaf(ScoreAtom(IntAtom(2))),
    //                 Leaf(ScoreAtom(IntAtom(7)))
    //             )), 
    //             Leaf(ScoreAtom(IntAtom(0)))
    //         ))
    //     )), 
    //     Leaf(ScoreAtom(IntAtom(0)))
    // ))

    traverse(0, Box::new(par.score_tree_iter()));
    par.sort();
    //println!("{:#?}", &par);



    
}

