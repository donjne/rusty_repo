# Day 3: Binary Trees & Modern Backend Architecture

## Objective

On Day 3, we delve into the fundamentals of binary trees and explore advanced backend architecture concepts. These tasks aim to enhance your problem-solving skills with recursive data structures while providing hands-on experience with modern backend technologies for scalable and efficient systems.

---

## Tasks Overview

### Binary Tree Implementation

1. **Implement a Binary Tree**  
2. **Add Insertion Operation**  
3. **Implement In-order, Pre-order, and Post-order Traversals**  
4. **Find Tree Height**  
5. **Check if Tree is Balanced**  
6. **Implement Tree Serialization/Deserialization**  
7. **Develop an Iterator for Tree Traversal**

### Backend Architecture

8. **Implement Async Runtime with Work Stealing**  
9. **Create Event-driven I/O Multiplexer**  
10. **Build Connection Pool with Backpressure**  
11. **Implement Circuit Breaker Pattern**  
12. **Create Rate Limiter with Token Bucket Algorithm**  
13. **Build Async Job Queue with Priorities**  
14. **Implement Distributed Task Scheduler**  
15. **Create Metrics Collection System**

---

## Detailed Task Descriptions

### Binary Trees

#### 1. Implement a Binary Tree

- **Objective**: Build a basic binary tree structure.  
- **Implementation**:  
  - Create a `Node` struct with `value`, `left`, and `right` fields.
  - Implement a `BinaryTree` struct to manage the root node.

---

#### 2. Add Insertion Operation

- **Objective**: Implement an insertion method to add elements to the binary tree.  
- **Implementation**:  
  - Recursively traverse the tree and place the new element at the correct position.

---

#### 3. Implement In-order, Pre-order, and Post-order Traversals

- **Objective**: Implement three types of tree traversals.  
- **Implementation**:  
  - In-order: Left -> Root -> Right  
  - Pre-order: Root -> Left -> Right  
  - Post-order: Left -> Right -> Root  

---

#### 4. Find Tree Height

- **Objective**: Calculate the height of the binary tree.  
- **Implementation**:  
  - Recursively determine the height of the left and right subtrees, returning the maximum of the two plus one.

---

#### 5. Check if Tree is Balanced

- **Objective**: Determine if the binary tree is balanced (height difference ≤ 1 for all nodes).  
- **Implementation**:  
  - Recursively calculate subtree heights and compare them.

---

#### 6. Implement Tree Serialization/Deserialization

- **Objective**: Enable saving and loading binary trees.  
- **Implementation**:  
  - Use pre-order traversal for serialization.
  - Deserialize by reconstructing the tree from the serialized data.

---

#### 7. Develop an Iterator for Tree Traversal

- **Objective**: Create an iterator for in-order traversal of the binary tree.  
- **Implementation**:  
  - Use a stack to simulate recursion and yield each node's value.

---

### Modern Backend Architecture

#### 8. Implement Async Runtime with Work Stealing

- **Objective**: Build an async runtime that supports work stealing for efficient task scheduling.  
- **Implementation**:  
  - Use thread-safe queues to balance work across threads dynamically.

---

#### 9. Create Event-driven I/O Multiplexer

- **Objective**: Handle multiple I/O operations asynchronously.  
- **Implementation**:  
  - Use libraries like `mio` or `tokio` to manage non-blocking I/O.

---

#### 10. Build Connection Pool with Backpressure

- **Objective**: Create a connection pool that supports limiting and controlling connections.  
- **Implementation**:  
  - Use bounded channels and backpressure mechanisms to manage load.

---

#### 11. Implement Circuit Breaker Pattern

- **Objective**: Add resilience to services by implementing a circuit breaker.  
- **Implementation**:  
  - Track failures and open the circuit when a threshold is reached.

---

#### 12. Create Rate Limiter with Token Bucket Algorithm

- **Objective**: Limit request rates using the token bucket algorithm.  
- **Implementation**:  
  - Use a timer-based system to refill tokens periodically.

---

#### 13. Build Async Job Queue with Priorities

- **Objective**: Create a job queue with priority-based task scheduling.  
- **Implementation**:  
  - Use a heap to manage tasks by priority.

---

#### 14. Implement Distributed Task Scheduler

- **Objective**: Build a scheduler for tasks across distributed nodes.  
- **Implementation**:  
  - Use message queues to coordinate task distribution.

---

#### 15. Create Metrics Collection System

- **Objective**: Collect and aggregate system metrics for monitoring.  
- **Implementation**:  
  - Use counters, histograms, and gauges to gather key metrics.

---

## Experience and Learnings

Day 3's tasks offer an exciting blend of theoretical and practical challenges:

1. **Recursive Thinking**: Mastering binary trees reinforces problem-solving with recursion.  
2. **Concurrency and Async Programming**: Backend tasks provide real-world experience in handling asynchronous operations and concurrency.  
3. **System Design**: Implementing patterns like circuit breakers and rate limiters demonstrates how to build resilient and scalable systems.  
4. **Performance Optimization**: Managing priorities, backpressure, and distributed tasks showcases advanced backend design principles.

Get ready for a day filled with trees, tasks, and technologies! 🚀
