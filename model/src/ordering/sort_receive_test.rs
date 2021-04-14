



#[test]
fn receive_should_be_sorted_basing_on_persistence_peek_channels_patterns_and_then_body() {
    use super::*;

    let mut par1 = Par::default();
    
    par1.receives.push(
        {
            let mut receive = Receive::default();
            receive.binds.push(
                {
                    let mut bind = ReceiveBind::default();
                    bind.patterns.push(
                        {
                            let mut parttern = Par::default();
                            parttern.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(1))
                            });
                            parttern
                        }
                    );
                    bind.source = Some(
                        {
                            let mut source = Par::default();
                            source.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(3))
                            });
                            source
                        }
                    );
                    bind
                }
            );
            receive.body = Some(Par::default());
            receive.persistent = false;
            receive.peek = false;
            receive.bind_count = 0;
            receive.connective_used = false;
            receive
        }
    );


    par1.receives.push(
        {
            let mut receive = Receive::default();
            receive.binds.push(
                {
                    let mut bind = ReceiveBind::default();
                    bind.patterns.push(
                        {
                            let mut parttern = Par::default();
                            parttern.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(0))
                            });
                            parttern
                        }
                    );
                    bind.source = Some(
                        {
                            let mut source = Par::default();
                            source.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(3))
                            });
                            source
                        }
                    );
                    bind
                }
            );
            receive.body = Some(
                {
                    let mut body = Par::default();
                    body.exprs.push(Expr {
                        expr_instance : Some(expr::ExprInstance::EVarBody(EVar{
                            v : Some(Var {
                                var_instance : Some(var::VarInstance::BoundVar(0))
                            })
                        }))
                    });
                    body
                }
            );
            receive.persistent = false;
            receive.peek = false;
            receive.bind_count = 0;
            receive.connective_used = false;
            receive
        }
    );


    par1.receives.push(
        {
            let mut receive = Receive::default();
            receive.binds.push(
                {
                    let mut bind = ReceiveBind::default();
                    bind.patterns.push(
                        {
                            let mut parttern = Par::default();
                            parttern.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(0))
                            });
                            parttern
                        }
                    );
                    bind.source = Some(
                        {
                            let mut source = Par::default();
                            source.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(3))
                            });
                            source
                        }
                    );
                    bind
                }
            );
            receive.body = Some(Par::default());
            receive.persistent = false;
            receive.peek = false;
            receive.bind_count = 0;
            receive.connective_used = false;
            receive
        }
    );


    par1.receives.push(
        {
            let mut receive = Receive::default();
            receive.binds.push(
                {
                    let mut bind = ReceiveBind::default();
                    bind.patterns.push(
                        {
                            let mut parttern = Par::default();
                            parttern.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(0))
                            });
                            parttern
                        }
                    );
                    bind.source = Some(
                        {
                            let mut source = Par::default();
                            source.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(3))
                            });
                            source
                        }
                    );
                    bind
                }
            );
            receive.body = Some(Par::default());
            receive.persistent = true;
            receive.peek = false;
            receive.bind_count = 0;
            receive.connective_used = false;
            receive
        }
    );


    par1.receives.push(
        {
            let mut receive = Receive::default();
            receive.binds.push(
                {
                    let mut bind = ReceiveBind::default();
                    bind.patterns.push(
                        {
                            let mut parttern = Par::default();
                            parttern.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(0))
                            });
                            parttern
                        }
                    );
                    bind.source = Some(
                        {
                            let mut source = Par::default();
                            source.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(3))
                            });
                            source
                        }
                    );
                    bind
                }
            );
            receive.body = Some(Par::default());
            receive.persistent = true;
            receive.peek = true;
            receive.bind_count = 0;
            receive.connective_used = false;
            receive
        }
    );

    par1.receives.push(
        {
            let mut receive = Receive::default();
            receive.binds.push(
                {
                    let mut bind = ReceiveBind::default();
                    bind.patterns.push(
                        {
                            let mut parttern = Par::default();
                            parttern.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(100))
                            });
                            parttern
                        }
                    );
                    bind.source = Some(
                        {
                            let mut source = Par::default();
                            source.exprs.push(Expr {
                                expr_instance : Some(expr::ExprInstance::GInt(2))
                            });
                            source
                        }
                    );
                    bind
                }
            );
            receive.body = Some(Par::default());
            receive.persistent = false;
            receive.peek = false;
            receive.bind_count = 0;
            receive.connective_used = false;
            receive
        }
    );


    par1.sort();
    let mut output = String::new();
    traverse(par1.score_tree_iter(), 0, &mut output);

    let target = "ScoreAtom::IntAtom(999)
{
  ScoreAtom::IntAtom(301)
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
  {
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(2)
      }
      ScoreAtom::IntAtom(0)
    }
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(100)
      }
      ScoreAtom::IntAtom(0)
    }
    ScoreAtom::IntAtom(0)
  }
  {
    ScoreAtom::IntAtom(999)
    ScoreAtom::IntAtom(0)
  }
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
}
{
  ScoreAtom::IntAtom(301)
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
  {
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(3)
      }
      ScoreAtom::IntAtom(0)
    }
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(0)
      }
      ScoreAtom::IntAtom(0)
    }
    ScoreAtom::IntAtom(0)
  }
  {
    ScoreAtom::IntAtom(999)
    ScoreAtom::IntAtom(0)
  }
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
}
{
  ScoreAtom::IntAtom(301)
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
  {
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(3)
      }
      ScoreAtom::IntAtom(0)
    }
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(0)
      }
      ScoreAtom::IntAtom(0)
    }
    ScoreAtom::IntAtom(0)
  }
  {
    ScoreAtom::IntAtom(999)
    {
      ScoreAtom::IntAtom(100)
      {
        ScoreAtom::IntAtom(50)
        ScoreAtom::IntAtom(0)
      }
    }
    ScoreAtom::IntAtom(0)
  }
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
}
{
  ScoreAtom::IntAtom(301)
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
  {
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(3)
      }
      ScoreAtom::IntAtom(0)
    }
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(1)
      }
      ScoreAtom::IntAtom(0)
    }
    ScoreAtom::IntAtom(0)
  }
  {
    ScoreAtom::IntAtom(999)
    ScoreAtom::IntAtom(0)
  }
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
}
{
  ScoreAtom::IntAtom(301)
  ScoreAtom::IntAtom(1)
  ScoreAtom::IntAtom(0)
  {
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(3)
      }
      ScoreAtom::IntAtom(0)
    }
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(0)
      }
      ScoreAtom::IntAtom(0)
    }
    ScoreAtom::IntAtom(0)
  }
  {
    ScoreAtom::IntAtom(999)
    ScoreAtom::IntAtom(0)
  }
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
}
{
  ScoreAtom::IntAtom(301)
  ScoreAtom::IntAtom(1)
  ScoreAtom::IntAtom(1)
  {
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(3)
      }
      ScoreAtom::IntAtom(0)
    }
    {
      ScoreAtom::IntAtom(999)
      {
        ScoreAtom::IntAtom(2)
        ScoreAtom::IntAtom(0)
      }
      ScoreAtom::IntAtom(0)
    }
    ScoreAtom::IntAtom(0)
  }
  {
    ScoreAtom::IntAtom(999)
    ScoreAtom::IntAtom(0)
  }
  ScoreAtom::IntAtom(0)
  ScoreAtom::IntAtom(0)
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

    assert!(output.eq(target));

}