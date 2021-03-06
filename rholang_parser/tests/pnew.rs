mod common;

use model::*;


fn validate_send( my_send : &RhoSend, ground_int : i64, bound_var_idx : i32, bitset_index : usize ) {
            
    let (chan_expr_inst, data_expr_inst) = match my_send {
        RhoSend {
            data : ref list,
            locally_free : Some(locally_free),
            connective_used : false,
            persistent : false,
            chan : Some(ref p),
        } => {
            let mut target_bitset = BitSet::default();
            target_bitset.insert(bitset_index);
            target_bitset.symmetric_difference_with(locally_free);
            assert_eq!(target_bitset.is_empty(), true);

            assert_eq!(list.len(), 1);
            let data_par = &list[0];
            assert_eq!(data_par.exprs.len(), 1);
            assert_eq!(p.exprs.len(), 1);
            (p.exprs[0].expr_instance.as_ref(), data_par.exprs[0].expr_instance.as_ref())
        },
        _ => panic!("{:#?}", my_send),
    };

    match data_expr_inst {
        Some( expr::ExprInstance::GInt(g) ) => {
            assert_eq!(ground_int, *g);
        },
        _ => panic!("{:#?}", data_expr_inst),
    };
    
    match chan_expr_inst {
        Some(
            expr::ExprInstance::EVarBody(
                EVar {
                    v : Some(
                        Var {
                            var_instance : Some(
                                var::VarInstance::BoundVar(i)
                            )
                        }
                    )
                }
            )
        ) => {
            assert_eq!(*i, bound_var_idx);
        },
        _ => panic!("{:#?}", chan_expr_inst),
    };
}

#[test]
fn pnew_should_bind_new_variables() {
    let rholang_code = "
    new x, y, z in {
        x!(7) | y!(8) | z!(9)
    }
    ";

    let mut normalizer = common::run_normalizer(rholang_code);
    let root_par = normalizer.par.take().unwrap();

    assert_eq!( root_par.news.len(), 1);
    let rho_new = &root_par.news[0];

    let sub_par = match rho_new {
        RhoNew {
            bind_count : 3,
            p : Some(ref p),
            uri,
            locally_free,
            ..
        } 
        if uri.len() == 0 &&
            ( 
                locally_free.is_none() ||
                locally_free.as_ref().unwrap().is_empty()
            )
        => p,
        _ => {
            panic!("{:#?}", rho_new);
        }
    };
    
    

    assert_eq!(sub_par.sends.len(), 3);
    validate_send(&sub_par.sends[0], 9, 0, 0);
    validate_send(&sub_par.sends[1], 8, 1, 1);
    validate_send(&sub_par.sends[2], 7, 2, 2);
}


#[test]
fn pnew_should_sort_uri_and_place_them_at_the_end() {
    let rholang_code = "
    new x, y, r(`rho:registry`), our(`rho:stdout`), z in {
        x!(7) | 
        y!(8) | 
        r!(9) |
        our!(10) |
        z!(11)
    }
    ";
    let mut normalizer = common::run_normalizer(rholang_code);
    let root_par = normalizer.par.take().unwrap();

    assert_eq!( root_par.news.len(), 1);
    let rho_new = &root_par.news[0];

    let sub_par = match rho_new {
        RhoNew {
            bind_count : 5,
            p : Some(ref p),
            uri : uris,
            locally_free,
            ..
        }  if locally_free.is_none() || locally_free.as_ref().unwrap().is_empty()
        => {
            assert_eq!( uris.len(), 2);
            assert!( uris.contains(&"rho:registry".to_string()) );
            assert!( uris.contains(&"rho:stdout".to_string()) );
            p
        },
        _ => {
            panic!("{:#?}", rho_new);
        }
    };

    assert_eq!(sub_par.sends.len(), 5);
    validate_send(&sub_par.sends[0], 11, 2, 2);
    validate_send(&sub_par.sends[1], 10, 0, 0);
    validate_send(&sub_par.sends[2], 9, 1, 1);
    validate_send(&sub_par.sends[3], 8, 3, 3);
    validate_send(&sub_par.sends[4], 7, 4, 4);

    let mut bitset = BitSet::new();
    bitset.insert(0);
    bitset.insert(1);
    bitset.insert(2);
    bitset.insert(3);
    bitset.insert(4);
    bitset.symmetric_difference_with(sub_par.locally_free.as_ref().unwrap());
    assert!( bitset.is_empty() );

}