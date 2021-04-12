use super::*;

#[cfg(test)] 
fn traverse(depth : usize, iter : Box<dyn Iterator<Item = Node<'_>> + '_>) {
    for node in iter {
        match node {
            Node::Leaf(scored_atom) => {
                match scored_atom {
                    ScoreAtom::IntAtom(i) => println!("{:indent$}ScoreAtom::IntAtom({})", "", i, indent=depth*2),
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
fn send_should_use_sorted_subtrees_and_their_scores_in_results() {
    let mut send1 = Send::default();
    send1.chan = Some(Par::default());

    let mut send2 = Send::default();
    {
        let mut receive = Receive::default();
        receive.body = Some(Par::default());
        let mut par = Par::default();
        par.receives.push(receive);
        send2.chan = Some(par);
    }

    let mut p12 = Par::default();
    p12.sends.push(send1.clone());
    p12.sends.push(send2.clone());

    // The score tree is something like
    // Node(List(
    //     Leaf(ScoreAtom(IntAtom(999))),
    //     Node(List(
    //         Leaf(ScoreAtom(IntAtom(300))),
    //         Leaf(ScoreAtom(IntAtom(0))),
    //         Node(List(
    //             Leaf(ScoreAtom(IntAtom(999))),
    //             Leaf(ScoreAtom(IntAtom(0)))
    //         )),
    //         Leaf(ScoreAtom(IntAtom(0)))
    //     )),
    //     Node(List(
    //         Leaf(ScoreAtom(IntAtom(300))),
    //         Leaf(ScoreAtom(IntAtom(0))),
    //         Node(List(
    //             Leaf(ScoreAtom(IntAtom(999))),
    //             Node(List(
    //                 Leaf(ScoreAtom(IntAtom(301))),
    //                 Leaf(ScoreAtom(IntAtom(0))),
    //                 Leaf(ScoreAtom(IntAtom(0))),
    //                 Node(List(
    //                     Leaf(ScoreAtom(IntAtom(999))),
    //                     Leaf(ScoreAtom(IntAtom(0)))
    //                 )),
    //                 Leaf(ScoreAtom(IntAtom(0))),
    //                 Leaf(ScoreAtom(IntAtom(0)))
    //             )),
    //             Leaf(ScoreAtom(IntAtom(0)))
    //         )),
    //         Leaf(ScoreAtom(IntAtom(0)))
    //     )),
    //     Leaf(ScoreAtom(IntAtom(0)))
    // ))
    
    traverse(0, Box::new(p12.score_tree_iter()));

    
}

