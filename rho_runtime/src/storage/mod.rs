mod space_matcher;
mod space_ops;
mod space;
mod hot_store;

pub trait ITuplespace { 
    fn consume();
    fn produce();
}

trait ISpace : ITuplespace {
    //fn create_checkpoint();
    //fn create_soft_checkpoint();

    fn reset();
    //fn get_data();
    //fn get_waiting_continuations();
    //fn clear();
    //fn to_map();
}

trait ISpaceMatcher : ISpace {
    fn find_matching_data_candidate();
    fn extract_data_candidates();
    fn extract_first_match();
}

trait ISpaceOps : ISpaceMatcher {
    fn produce_with_lock();
}





