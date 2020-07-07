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

    pub fn from_parent(parent: &Thread) -> Thread {
        let mut th = Thread::default();
        th.pthid = parent.thid.clone();
        th
    }

    pub fn set_thid(mut self, thid: String) -> Thread {
        self.thid = Some(thid);
        self
    }

    pub fn set_pthid(mut self, pthid: String) -> Thread {
        self.pthid = Some(pthid);
        self
    }

    pub fn set_opt_pthid(mut self, pthid: Option<String>) -> Thread {
        self.pthid = pthid;
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

    pub fn set_pthid(mut self, thid: String) -> Thread {
        self.pthid = Some(thid);
        self
    }

    pub fn is_reply(&self, id: &str) -> bool {
        self.thid.clone().unwrap_or_default() == id.to_string()
    }

    pub fn check_message_order(&self, sender: &str, received_message_thread: &Thread) -> VcxResult<()> {
        let expected_order = match self.received_orders.get(sender).cloned() {
            Some(order) => order + 1,
            None => 0
        };

        if received_message_thread.sender_order != expected_order {
            warn!("Message is out of order. Expected sender_order: {}, Received sender_order: {}",
                  expected_order, received_message_thread.sender_order);
            return Ok(()); // TODO: return error once we sure other clients control threading proper
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

#[cfg(test)]
mod tests {
    use super::*;

    const THID: &str = "id";

    #[test]
    fn test_thread_new() {
        let thread = Thread::new();
        let expected = Thread {
            thid: None,
            pthid: None,
            sender_order: 0,
            received_orders: HashMap::new(),
        };
        assert_eq!(expected, thread);
    }

    #[test]
    fn test_thread_set_thid() {
        let thread = Thread::new()
            .set_thid(THID.to_string());

        assert_eq!(THID, thread.thid.unwrap());
    }

    #[test]
    fn test_thread_set_sender_order() {
        let sender_order = 1;

        let thread = Thread::new()
            .set_sender_order(sender_order);

        assert_eq!(sender_order, thread.sender_order);
    }

    #[test]
    fn test_thread_increment_sender_order() {
        let thread = Thread::new();
        assert_eq!(0, thread.sender_order);

        let thread = thread.increment_sender_order();
        assert_eq!(1, thread.sender_order);
    }

    #[test]
    fn test_thread_update_received_order() {
        let did = "qwertytyyu";

        let thread = Thread::new();
        assert_eq!(None, thread.received_orders.get(did));

        let thread = thread.update_received_order(did);
        assert_eq!(Some(0), thread.received_orders.get(did).cloned());

        let thread = thread.update_received_order(did);
        assert_eq!(Some(1), thread.received_orders.get(did).cloned());
    }

    #[test]
    fn test_thread_is_reply() {
        let thread = Thread::new();
        assert!(!thread.is_reply(THID));

        let thread = Thread::new()
            .set_thid(THID.to_string());

        assert!(thread.is_reply(THID));
        assert!(!thread.is_reply("other"));
    }

    #[test]
    fn test_thread_check_message_order() {
        let did = "qwertytyyu";

        let thread = Thread::new()
            .set_thid(THID.to_string());

        let received_thread = thread.clone()
            .set_sender_order(0);

        // thread.received orders = {}  and received_thread.sender_order = 0
        thread.check_message_order(did, &received_thread).unwrap();

        // thread.received orders = {did: 0}  and received_thread.sender_order = 1
        let thread = thread.update_received_order(did);
        let received_thread = received_thread.clone().increment_sender_order();
        thread.check_message_order(did, &received_thread).unwrap();

        // thread.received orders = {did: 1}  and received_thread.sender_order = 2
        let thread = thread.update_received_order(did);
        let received_thread = received_thread.clone().increment_sender_order();
        thread.check_message_order(did, &received_thread).unwrap();

        // thread.received orders = {did: 2}  and received_thread.sender_order = 2
        let thread = thread.update_received_order(did);
        thread.check_message_order(did, &received_thread).unwrap(); // FIXME: it will fail once we update `check_message_order` to throw error
    }
}