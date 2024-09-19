// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::error;
use std::sync::mpsc::{channel, sync_channel, Receiver, RecvError, Sender, SyncSender, };

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

#[derive(Debug, thiserror::Error)]
#[error("the store is overloaded")]
pub struct OverloadedError;

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadedError> {
        let (resp_sender, resp_recv) = channel();
        self.sender.try_send(Command::Insert { 
            draft: draft, 
            response_channel: resp_sender 
        }).map_err(|_| OverloadedError)?;
        Ok(resp_recv.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, OverloadedError> {
        let (resp_sender, resp_recv) = channel();
        let _ = self.sender.send(Command::Get { 
            id, 
            response_channel: resp_sender 
        }).map_err(|_| OverloadedError)?;
        Ok(resp_recv.recv().unwrap())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
