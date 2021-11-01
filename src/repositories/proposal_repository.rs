use crate::{db::Client, models::proposal::Proposal};

pub struct ProposalRepository {
    client: Client,
}

impl ProposalRepository {
    fn get(id: &str) -> Proposal {
        Proposal {
            title: todo!(),
            description: todo!(),
            created: todo!(),
            updated: todo!(),
            comments: todo!(),
            owner: todo!(),
            upvotes: todo!(),
        }
    }
}
