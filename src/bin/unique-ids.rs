use std::fmt::format;

// cargo b && ../maelstrom/maelstrom test -w unique-ids --bin target/debug/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
use anyhow::anyhow;

use maelstrom_echo::{Body, Message, Node};

struct UniqueIdNode {
    id: String,
    msg_count: usize,
}

impl Node for UniqueIdNode {
    fn new(id: String, _node_ids: Vec<String>) -> Self {
        UniqueIdNode { id, msg_count: 0 }
    }

    fn handle(&mut self, msg: Message) -> anyhow::Result<()> {
        match msg.body {
            Body::Generate { msg_id } => {
                let id = format!("{}:{}", self.id, self.msg_count);
                let response = Message {
                    src: self.id.clone(),
                    dest: msg.src,
                    body: Body::GenerateOk {
                        id,
                        in_reply_to: msg_id,
                    },
                };
                self.msg_count += 1;
                self.send(response)
            }

            _ => Err(anyhow!("unexpected message: {msg:?}")),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut node = UniqueIdNode::initialize()?;
    node.run()
}
