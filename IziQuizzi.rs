use async_std::sync::{RwLock, RwLockWriteGuard};
use async_trait::async_trait;
use futures::{future, select};
use std::{
  collections::{HashMap, HashSet, VecDeque},
  net::IpAddr,
  ptr::null,
};
use uuid::Uuid;

use crate::{
  core::{MessageServer, SpamChecker, MAILBOX_SIZE},
  messages::{
    ClientError, ClientId, ClientMessage, ClientPollReply, ClientReply, FullyQualifiedMessage,
    Sequence, ServerId,
  },
};

use crate::messages::{Outgoing, ServerMessage, ServerReply};

// this structure will contain the data you need to track in your server
// this will include things like delivered messages, clients last seen sequence number, etc.
pub struct Server<C: SpamChecker> {
  id: ServerId,
  checker: C,
  clients: RwLock<HashMap<ClientId, Clientinfo>>, // add things here
}

pub struct Clientinfo {
  client_id: ClientId,
  src_ip: IpAddr,
  name: String,
  seq_id: u128,
  mailbox:VecDeque<ClientMessage>
}

#[async_trait]
impl<C: SpamChecker + Send + Sync> MessageServer<C> for Server<C> {
  const GROUP_NAME: &'static str = "Martinez Alexandre, Louis Vernanchet";

  fn new(checker: C, id: ServerId) -> Self {
    Server {
      id,
      checker,
      clients: RwLock::new(HashMap::new()),
    }
  }

  // note: you need to roll a Uuid, and then convert it into a ClientId
  // Uuid::new_v4() will generate such a value
  // you will most likely have to edit the Server struct as as to store information about the client
  //
  // for spam checking, you will need to run both checks in parallel, and take a decision as soon as
  // each checks return
  async fn register_local_client(&self, src_ip: IpAddr, name: String) -> Option<ClientId> {
    let uuid: Uuid = Uuid::new_v4();
    let client_id = ClientId(uuid);
    let client_info = Clientinfo {
      client_id,
      src_ip,
      name,
      seq_id: 0,
      mailbox:VecDeque::new()
    };
    self.clients.write().await.insert(client_id, client_info);
    Some(client_id)
  }

  /*
   if the client is known, its last seen sequence number must be verified (and updated)
  */
  async fn handle_sequenced_message<A: Send>(
    &self,
    sequence: Sequence<A>,
  ) -> Result<A, ClientError> {
    match self.clients.write().await.get_mut(&sequence.src) {
      Some(info) =>{ 
        info.seq_id = sequence.seqid;
        Ok(sequence.content)
      },
      None => Err(ClientError::UnknownClient)
    }
  }

  /* Here client messages are handled.
    * if the client is local,
      * if the mailbox is full, BoxFull should be returned
      * otherwise, Delivered should be returned
    * if the client is unknown, the message should be stored and Delayed must be returned
    * (federation) if the client is remote, Transfer should be returned

    It is recommended to write an function that handles a single message and use it to handle
    both ClientMessage variants.
  */

  async fn handle_client_message(&self, src: ClientId, msg: ClientMessage) -> Vec<ClientReply> {
    match self.clients.read().await.get(&src){
      Some(info) =>{ 
        match msg {
            ClientMessage::Text { dest, content } => {

            },
            ClientMessage::MText { dest, content } => todo!()
        }
      },
      None => todo!()
  }
}


  /* for the given client, return the next message or error if available
   */
  async fn client_poll(&self, client: ClientId) -> ClientPollReply {
    todo!()
  }

  /* For announces
     * if the route is empty, return EmptyRoute
     * if not, store the route in some way
     * also store the remote clients
     * if one of these remote clients has messages waiting, return them
    For messages
     * if local, deliver them
     * if remote, forward them
  */
  async fn handle_server_message(&self, msg: ServerMessage) -> ServerReply {
    todo!()
  }

  async fn list_users(&self) -> HashMap<ClientId, String> {
    todo!()
  }

  // return a route to the target server
  // bonus points if it is the shortest route
  async fn route_to(&self, destination: ServerId) -> Option<Vec<ServerId>> {
    todo!()
  }
}

impl<C: SpamChecker + Sync + Send> Server<C> {
  // write your own methods here
}

fn helper_handle_client_message( src: ClientId, msg: ClientMessage) -> ClientReply {
  todo!()
}

#[cfg(test)]
mod test {
  use crate::testing::{test_message_server, TestChecker};

  use super::*;

  #[test]
  fn tester() {
    test_message_server::<Server<TestChecker>>();
  }
}
