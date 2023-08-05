// cargo b && ../maelstrom/maelstrom test -w unique-ids --bin target/debug/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
use anyhow::anyhow;

use maelstrom_echo::{Body, Message, Node};

struct BroadcastNode {
    id: String,
    messages: Vec<usize>,
}

impl Node for BroadcastNode {
    fn new(id: String, _node_ids: Vec<String>) -> Self {
        BroadcastNode {
            id,
            messages: vec![],
        }
    }

    fn handle(&mut self, msg: Message) -> anyhow::Result<()> {
        eprintln!("Received {msg:?}");
        match msg.body {
            Body::Topology {
                msg_id,
                topology: _topology,
            } => {
                let response = Message {
                    src: self.id.clone(),
                    dest: msg.src,
                    body: Body::TopologyOk {
                        in_reply_to: msg_id,
                    },
                };
                eprintln!("sending {response:?}");
                self.send(response)
            }
            Body::Broadcast { msg_id, message } => {
                self.messages.push(message);
                let response = Message {
                    src: self.id.clone(),
                    dest: msg.src,
                    body: Body::BroadcastOk {
                        in_reply_to: msg_id,
                    },
                };
                eprintln!("sending {response:?}");
                self.send(response)
            }
            Body::Read { msg_id } => {
                let response = Message {
                    src: self.id.clone(),
                    dest: msg.src,
                    body: Body::ReadOk {
                        in_reply_to: msg_id,
                        messages: self.messages.clone(),
                    },
                };
                eprintln!("sending {response:?}");
                self.send(response)
            }
            _ => Err(anyhow!("unexpected message: {msg:?}")),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut node = BroadcastNode::initialize()?;
    node.run()
}
