

#[test]
fn send_should_use_sorted_subtrees_and_their_scores_in_results() {
    use super::*;


    
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

    let mut p21 = Par::default();
    p21.sends.push(send2.clone());
    p21.sends.push(send1.clone());

    p12.sort();
    p21.sort();

    println!("{:#?}", &p21);



    
}

