# HLC Distributed System Prototype

This project implements a **distributed system** using **Hybrid Logical Clocks (HLCs)** in Rust.  
HLCs combine **physical time** and **logical clocks** to ensure causality and event ordering, even across distributed nodes.

## 📜 Overview

In distributed systems, clock synchronization is challenging due to **network delays** and **clock skew**.

- **Hybrid Logical Clocks (HLCs)** solve this by using physical time when available and falling back to logical counters for causality.
- This prototype demonstrates how multiple nodes can communicate and maintain a consistent view of events using HLCs.

## 📚 Features

- **Node communication** with message passing.
- **HLC implementation** to track and maintain causal order.
- **Thread-based simulation** for concurrent node operations.
- Optional **structured logging** with `log` and `env_logger`.

## 📂 Project Structure

## 🛠️ Setup and Installation

### Prerequisites

- **Rust**: Install Rust by running:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

git clone https://github.com/davidrhodus/hlc_prototype.git
cd hlc_prototype
