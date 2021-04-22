use super::*;
use super::super::rho_types::g_unforgeable::UnfInstance;

impl<'a> Scorable<'a> for &'a GUnforgeable {
    fn score_tree_iter(self) -> ScoreTreeIter<'a> {
        let next_func = match self.unf_instance {
            Some(UnfInstance::GPrivateBody(_)) => {
                UnforgeableScoreTreeIter::private_body_next
            },
            Some(UnfInstance::GDeployerIdBody(_)) => {
                UnforgeableScoreTreeIter::deployer_id_next
            },
            Some(UnfInstance::GDeployIdBody(_)) => {
                UnforgeableScoreTreeIter::deploy_id_next
            },
            Some(UnfInstance::GSysAuthTokenBody(_)) => {
                UnforgeableScoreTreeIter::sys_auth_token_next
            },
            None => {
                UnforgeableScoreTreeIter::absent_next
            }
        };
        UnforgeableScoreTreeIter{
            term : self,
            stage : 0,
            next_func : next_func,
        }.into()
    }
}

impl<'a> From<UnforgeableScoreTreeIter<'a>> for ScoreTreeIter<'a> {
    #[inline]
    fn from(inner: UnforgeableScoreTreeIter<'a>) -> ScoreTreeIter<'a> {
        ScoreTreeIter::Unforgeable(inner)
    }
}

pub(super) struct UnforgeableScoreTreeIter<'a> {
    pub term : &'a GUnforgeable,
    stage : u16,
    next_func : fn(&mut UnforgeableScoreTreeIter<'a>) -> Option<Node<'a>>,
}


impl<'a> Iterator for UnforgeableScoreTreeIter<'a> {
    type Item = Node<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        (self.next_func)(self)
    }
}


// case BoundVar(level) => Leaves(Score.BOUND_VAR, level.toLong)
// case FreeVar(level)  => Leaves(Score.FREE_VAR, level.toLong)
// case Wildcard(_)     => Leaves(Score.WILDCARD)
// case Empty           => Leaf(Score.ABSENT)

impl<'a> UnforgeableScoreTreeIter<'a> {

    fn private_body_next(&mut self) -> Option<Node<'a>> {
        match self.stage {
            0 => {
                self.stage += 1;
                Some(Node::Leaf(ScoreAtom::IntAtom(Score::PRIVATE as i64)))
            },

            1 => {
                self.stage += 1;
                if let Some(UnfInstance::GPrivateBody(GPrivate { id })) = &self.term.unf_instance {
                    Some(Node::Leaf(ScoreAtom::BytesAtom(&id[..])))
                } else {
                    None
                }
            },

            _ => None
        }
    }


    fn deployer_id_next(&mut self) -> Option<Node<'a>> {
        match self.stage {
            0 => {
                self.stage += 1;
                Some(Node::Leaf(ScoreAtom::IntAtom(Score::DEPLOYER_AUTH as i64)))
            },

            1 => {
                self.stage += 1;
                if let Some(UnfInstance::GDeployerIdBody(GDeployerId { public_key })) = &self.term.unf_instance {
                    Some(Node::Leaf(ScoreAtom::BytesAtom(&public_key[..])))
                } else {
                    None
                }
            },

            _ => None
        }
    }

    fn deploy_id_next(&mut self) -> Option<Node<'a>> {
        match self.stage {
            0 => {
                self.stage += 1;
                Some(Node::Leaf(ScoreAtom::IntAtom(Score::DEPLOY_ID as i64)))
            },

            1 => {
                self.stage += 1;
                if let Some(UnfInstance::GDeployIdBody(GDeployId { sig })) = &self.term.unf_instance {
                    Some(Node::Leaf(ScoreAtom::BytesAtom(&sig[..])))
                } else {
                    None
                }
            },

            _ => None
        }
    }


    fn sys_auth_token_next(&mut self) -> Option<Node<'a>> {
        match self.stage {
            0 => {
                self.stage += 1;
                Some(Node::Leaf(ScoreAtom::IntAtom(Score::SYS_AUTH_TOKEN as i64)))
            },

            _ => None
        }
    }

    fn absent_next(&mut self) -> Option<Node<'a>> {
        match self.stage {
            0 => {
                self.stage += 1;
                Some(Node::Leaf(ScoreAtom::IntAtom(Score::ABSENT as i64)))
            },

            _ => None
        }
    }



}

