





#[test]
fn news_should_be_sorted_basing_on_bind_count_urls_and_then_body() {
    use super::*;

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


    par.sort();
    let mut output = String::new();
    traverse(par.score_tree_iter(), 0, &mut output);

    let target = "ScoreAtom::IntAtom(999)
{
  ScoreAtom::IntAtom(303)
  ScoreAtom::IntAtom(1)
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
  {
    ScoreAtom::IntAtom(999)
    ScoreAtom::IntAtom(0)
  }
}
{
  ScoreAtom::IntAtom(303)
  ScoreAtom::IntAtom(2)
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
  {
    ScoreAtom::IntAtom(999)
    ScoreAtom::IntAtom(0)
  }
}
{
  ScoreAtom::IntAtom(303)
  ScoreAtom::IntAtom(2)
  ScoreAtom::StringAtom(rho:io:stderr)
  ScoreAtom::IntAtom(0)
  {
    ScoreAtom::IntAtom(999)
    ScoreAtom::IntAtom(0)
  }
}
{
  ScoreAtom::IntAtom(303)
  ScoreAtom::IntAtom(2)
  ScoreAtom::StringAtom(rho:io:stdout)
  ScoreAtom::IntAtom(0)
  {
    ScoreAtom::IntAtom(999)
    ScoreAtom::IntAtom(0)
  }
}
{
  ScoreAtom::IntAtom(303)
  ScoreAtom::IntAtom(2)
  ScoreAtom::StringAtom(rho:io:stdout)
  ScoreAtom::IntAtom(0)
  {
    ScoreAtom::IntAtom(999)
    {
      ScoreAtom::IntAtom(2)
      ScoreAtom::IntAtom(7)
    }
    ScoreAtom::IntAtom(0)
  }
}
ScoreAtom::IntAtom(0)
";
    
    
        fn traverse(iter : ScoreTreeIter<'_>, depth : usize, output : &mut String ) {
            for node in iter {
                match node {
                    Node::Leaf(scored_atom) => {
                        match scored_atom {
                            ScoreAtom::IntAtom(i) =>
                                output.push_str(&format!("{:indent$}ScoreAtom::IntAtom({})\n", "", i, indent=depth*2) ),
                            ScoreAtom::StringAtom(s) =>
                                output.push_str(&format!("{:indent$}ScoreAtom::StringAtom({})\n", "", s, indent=depth*2)),
                            _ => { panic!("Unknown type") },
                        }
                        
                    },
                    Node::Children(iter) => {
                        output.push_str(&format!("{:indent$}{{\n", "", indent=depth*2));
                        traverse(iter, depth+1, output);
                        output.push_str(&format!("{:indent$}}}\n", "", indent=depth*2));
                    }
                }
                
            }
        }
    
        println!("{}", &output);
        assert!(output.eq(target));

    
}

