use std::collections::HashMap;
use error::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Thread {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pthid: Option<String>,
    #[serde(default)]
    pub sender_order: u32,
    #[serde(default)]
    pub received_orders: HashMap<String, u32>,
}

impl Thread {
    pub fn new() -> Thread {
        Thread::default()
    }

    pub fn set_thid(mut self, thid: String) -> Thread {
        self.thid = Some(thid);
        self
    }

    pub fn set_sender_order(mut self, order: u32) -> Thread {
        self.sender_order = order;
        self
    }

    pub fn increment_sender_order(mut self) -> Thread {
        self.sender_order += 1;
        self
    }

    pub fn update_received_order(mut self, did: &str) -> Thread {
        self.received_orders.entry(did.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(0);
        self
    }

    pub fn increment_receiver(&mut self, did: &str) {
        self.received_orders.entry(did.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(0);
    }

    pub fn is_reply(&self, id: &str) -> bool {
        self.thid.clone().unwrap_or_default() == id.to_string()
    }

    pub fn check_message_order(&self, sender: &str, received_message_thread: &Thread) -> VcxResult<()> {
        let order = self.received_orders.get(sender).cloned().unwrap_or_default();
        let expected_order = order + 1;

        if received_message_thread.sender_order != expected_order {
            warn!("Message is out of order. Expected sender_order: {}, Received sender_order: {}",
                  expected_order, received_message_thread.sender_order);
            return Ok(()) // TODO: return error once we sure other clients control threading proper
        }
        Ok(())
    }
}

impl Default for Thread {
    fn default() -> Thread {
        Thread {
            thid: None,
            pthid: None,
            sender_order: 0,
            received_orders: HashMap::new(),
        }
    }
}

#[macro_export]
macro_rules! threadlike (($type:ident) => (
    impl $type {
        pub fn set_thread(mut self, thread: Thread) -> $type {
            self.thread = thread;
            self
        }

        pub fn set_thread_id(mut self, id: &str) -> $type {
            self.thread.thid = Some(id.to_string());
            self
        }

        pub fn update_received_order(mut self, did: &str) -> $type {
            self.thread = self.thread.update_received_order(did);
            self
        }

        pub fn from_thread(&self, id: &str) -> bool {
            self.thread.is_reply(id)
        }
    }
));