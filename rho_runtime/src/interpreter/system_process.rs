use super::*;
use smallvec::SmallVec;
use rustc_hash::FxHashMap;
use bytes::Bytes;


pub enum SystemProcessKind {
    Stdout = 0,
}


pub trait SystemProcess {
    fn kind() -> SystemProcessKind;
    fn name() -> String;
    fn execute(data_list : SmallVec<[ListParWithRandom; 3]>) -> ();
}


lazy_static! {
    static ref HASHMAP: FxHashMap<String, Par> = {
        let mut map = FxHashMap::default();
        map.insert(StdoutProcess::name(), create_par(StdoutProcess::kind()));
        map
    };
}

pub fn get_map() -> FxHashMap<String, Par> {
    HASHMAP.clone()
}


pub async fn setup<S>(storage : &S) where S : Storage + std::marker::Send + std::marker::Sync + 'static {
    let bind_pattern = BindPattern {
        remainder : None,
        free_count : 1,
        patterns : vec![Par::default()],
    };
    storage.install(create_par(StdoutProcess::kind()),
        bind_pattern,
        StdoutProcess::execute
    ).await;
}

fn create_par(kind : SystemProcessKind) -> Par {
    let arr : [u8; 1] = [kind as u8];
    let mut par = Par::default();
    par.unforgeables.push(
        {
            GUnforgeable {
                unf_instance : Some(g_unforgeable::UnfInstance::GPrivateBody(
                    GPrivate {
                        id : Bytes::copy_from_slice(&arr)
                    }
                ))
            }
        }
    );
    par
}


pub struct StdoutProcess {
}

impl SystemProcess for StdoutProcess {
    fn kind() -> SystemProcessKind {  SystemProcessKind::Stdout }
    fn name() -> String { "rho:io:stdout".to_owned() }


    fn execute(data_list : SmallVec<[ListParWithRandom; 3]>) -> () {
        if data_list.len() == 1 {
            let first_data = &data_list[0];
            if first_data.pars.len() == 1 {
                let first_par = &first_data.pars[0];
                if first_par.exprs.len() == 1 {
                    match &first_par.exprs[0] {
                        
                        Expr {  expr_instance: Some(expr::ExprInstance::GBool(b)) } => {
                            println!("stdout : {}", b);
                            return;
                        },
                        Expr {  expr_instance: Some(expr::ExprInstance::GInt(i)) } => {
                            println!("stdout : {}", i);
                            return;
                        },
                        Expr {  expr_instance: Some(expr::ExprInstance::GString(text)) } => {
                            println!("stdout : \"{}\"", text);
                            return;
                        },
                        _ => (),
                    }
                }
            }
            
        }
        println!("stdout : {:?}", data_list);
    }
}