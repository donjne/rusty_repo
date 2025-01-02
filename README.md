# 30 Day Rust Challenge

Welcome to my rigorous journey through Rust, Data Structures, Algorithms, Systems Programming, and Advanced Engineering concepts. This challenge is designed cover a vast spectrum of topics from basic programming to cutting-edge system architectures.

## Prerequisites

- **Basic Rust Syntax**: Familiarity with Rust's syntax, ownership model, and basic programming constructs.
- **Cargo**: Comfortable with creating and managing Rust projects.
- **Data Structures**: Understanding of arrays, vectors, strings, and basic algorithms.
- **Programming Fundamentals**: Strong grasp of programming concepts.
- **Command Line & Git**: Basic proficiency in using command line tools and version control.
- **Distributed Systems**: Preliminary knowledge of distributed system principles.

## Program Philosophy

This challenge merges two educational paths:

1. **Rust and Data Structures & Algorithms (DSA)** - Mastery of Rust-specific features, memory safety, and algorithm implementation.
2. **Advanced Engineering** - Dive into modern backend, distributed systems, blockchain, and performance optimization.

## Structure

- **Daily Challenges**: Each day includes 7-10 tasks of increasing complexity.
- **Focus Areas**
  - Rust-specific optimizations
  - Systems programming
  - Concurrency and parallelism
  - Backend architecture
  - Distributed systems and blockchain

## Daily Breakdown

**Day 1 to Day 30**: Each day's tasks are listed in a dedicated file, `DAY_X/README.md`, where X represents the day number. These files detail:

- **Basic Data Structures**
- **Advanced Data Structures**
- **Graph Algorithms**
- **String and Bit Manipulation**
- **Network and System Programming**
- **Database Systems**
- **Blockchain and Consensus Mechanisms**
- **Concurrency and Parallelism**
- **Security and Performance Optimization**

---

## How to Use This Repository

### 1. Fork or Clone

Clone this repository to your local machine or fork it to your own GitHub account for modifications:

```bash
git clone https://github.com/donjne/rusty_repo.git
```

### 2. Setup Rust Environment

Install Rust:
If you haven't installed Rust yet, use the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Update the Rust Toolchain:
Ensure you have the latest version:

```bash
rustup update
```

### 3. Daily Routine

Read the Instructions:
Open the DAY_X.md file to review the tasks for the day.

Implement the Tasks:
Create a new Rust project or add tasks to an existing project using Cargo:

```bash
cargo new task_name
```

```bash
cd task_name
```

Follow best practices while writing Rust code.

Test Your Code:
Write unit tests for your implementations. Run tests using:

```bash
cargo test
```

Document Your Code:
Add comments to explain complex parts or optimizations. Use /// for documentation comments and generate docs using:

```bash
cargo doc --open
```

### 4. Submit Your Work

Push to Your Fork:
Save your work to your GitHub repository:

```bash
git add .
git commit -m "Day X: Implemented tasks"
git push origin master
```

Use Branches for Each Day:
Keep your repository organized by using branches:

```bash
git checkout -b day_X
git add .
git commit -m "Day X: Implemented tasks"
git push origin day_X
```

### 5. Review and Learn

Compare your implementation with reference solutions (if provided).
Engage in discussions with the Rust community or your learning group to get feedback.
Reflect on what you’ve learned and identify areas for improvement.

## Contribution Guidelines

Issues:
Report bugs, suggest improvements, or discuss ideas in the Issues section.

Pull Requests:
Propose changes or submit new challenges. Ensure they align with the challenge's goals.

## Code Style

Follow Rust's standards. Use rustfmt for consistent formatting:

```bash
cargo fmt
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgements

Rust Community: For providing an exceptional ecosystem and learning resources.
Contributors: For shaping this challenge and adding value to the learning journey.

## Final Project

On the last day, integrate your learning into a comprehensive project. The final project should demonstrate:

Documentation: Detailed architecture and design decisions.
Performance: Benchmarks and optimization reports.
Security: A security audit checklist.
Scalability: Evidence of scalability testing.
Recovery: Strategies for failure recovery.
Monitoring: Implementation of observability tools.
Embark on this journey to not just learn Rust, but master the art of engineering with it! 🚀

## Project Structure

The project is organized as follows:

```css
30-day-rust-challenge/
│
├── day_one/
│   ├── task_01_stack/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   ├── Cargo.toml
│   ├── task_02_queue/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   ├── Cargo.toml
│   └── README.md
│
├── day_two/
│   ├── task_01_circular_buffer/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   ├── Cargo.toml
│   └── README.md
│
└── README.md
```

In this structure, day_one, day_two, etc., are folders for each day's tasks.
Inside each day's folder, you will find subfolders for individual tasks (e.g., task_01_stack, task_02_queue).
Each task has its own Rust project with Cargo.toml and src/main.rs.
A README.md file for each day to guide you through the tasks.
