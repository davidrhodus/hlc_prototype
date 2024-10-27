use std::cmp;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use rand::Rng;
use log::info;
use env_logger;

/// Hybrid Logical Clock (HLC) struct
#[derive(Clone, Debug)]
struct HybridLogicalClock {
    physical_time: u64,
    logical_counter: u64,
}

impl HybridLogicalClock {
    /// Create a new HLC initialized to the current time
    fn new() -> Self {
        HybridLogicalClock {
            physical_time: current_millis(),
            logical_counter: 0,
        }
    }

    /// Increment the clock on a local event
    fn increment(&mut self) {
        let now = current_millis();
        if now == self.physical_time {
            self.logical_counter += 1;
        } else {
            self.physical_time = now;
            self.logical_counter = 0;
        }
    }

    /// Update the clock on receiving a message
    fn update(&mut self, remote: &HybridLogicalClock) {
        let now = current_millis();
        self.physical_time = cmp::max(now, remote.physical_time);

        if self.physical_time == remote.physical_time {
            self.logical_counter = cmp::max(self.logical_counter, remote.logical_counter) + 1;
        } else {
            self.logical_counter = 0;
        }
    }

    /// Get the current timestamp
    fn get_time(&self) -> (u64, u64) {
        (self.physical_time, self.logical_counter)
    }
}

/// Get the current time in milliseconds since the UNIX epoch
fn current_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

/// Node struct to represent a distributed node
struct Node {
    id: u32,
    clock: HybridLogicalClock,
    sender: mpsc::Sender<Message>,
}

impl Node {
    fn new(id: u32, sender: mpsc::Sender<Message>) -> Self {
        Node {
            id,
            clock: HybridLogicalClock::new(),
            sender,
        }
    }

    /// Simulate sending a message to another node
    fn send_message(&mut self, target_id: u32) {
        self.clock.increment();
        let timestamp = self.clock.get_time();
        println!(
            "Node {} sending message with HLC {:?} to Node {}",
            self.id, timestamp, target_id
        );
        self.sender
            .send(Message {
                sender_id: self.id,
                target_id,
                timestamp: self.clock.clone(),
            })
            .expect("Failed to send message");
    }

    /// Handle an incoming message
    fn handle_message(&mut self, message: Message) {
        println!(
            "Node {} received message from Node {} with HLC {:?}",
            self.id, message.sender_id, message.timestamp
        );
        self.clock.update(&message.timestamp);
        println!("Node {} updated HLC to {:?}", self.id, self.clock.get_time());
    }
}

/// Message struct to represent a message between nodes
#[derive(Clone, Debug)]
struct Message {
    sender_id: u32,
    target_id: u32,
    timestamp: HybridLogicalClock,
}

/// Simulate the network communication between nodes
fn simulate_network() {
    // Channels for message passing
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // Create two nodes
    let mut node1 = Node::new(1, tx.clone());
    let mut node2 = Node::new(2, tx.clone());

    // Thread to simulate Node 1 behavior
    let rx1 = Arc::clone(&rx);
    thread::spawn(move || loop {
        let delay = rand::thread_rng().gen_range(1..=3);
        thread::sleep(Duration::from_secs(delay));
        node1.send_message(2);

        if let Ok(message) = rx1.lock().unwrap().recv() {
            if message.target_id == 1 {
                node1.handle_message(message);
            }
        }
    });

    // Thread to simulate Node 2 behavior
    let rx2 = Arc::clone(&rx);
    thread::spawn(move || loop {
        let delay = rand::thread_rng().gen_range(1..=3);
        thread::sleep(Duration::from_secs(delay));
        node2.send_message(1);

        if let Ok(message) = rx2.lock().unwrap().recv() {
            if message.target_id == 2 {
                node2.handle_message(message);
            }
        }
    });

    // Keep the main thread alive
    loop {
        thread::sleep(Duration::from_secs(10));
    }
}

fn main() {
        env_logger::init();
    info!("Starting the HLC distributed system...");
    simulate_network();
}
