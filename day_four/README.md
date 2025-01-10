# Day 4: Binary Search Trees & Network Programming

## Objective

On Day 4, we dive deep into binary search trees (BSTs) and network programming. This combination of data structures and systems programming tasks will enhance your ability to design efficient algorithms and build robust networked applications. The day is filled with challenges aimed at balancing theory with practical implementation, paving the way for scalable and high-performance systems.

---

## Tasks Overview

### Binary Search Trees (BST)

1. **Implement BST Insertion**  
2. **Add BST Search**  
3. **Implement BST Deletion**  
4. **Find BST Successor**  
5. **Validate BST Properties**  
6. **Implement AVL Tree Rotations**  
7. **Add Balancing to BST**

### Network Programming

8. **Build Async TCP Server with Custom Protocol**  
9. **Implement HTTP/2 Server with Streaming**  
10. **Create QUIC Protocol Implementation**  
11. **Build gRPC Service with Bidirectional Streaming**  
12. **Implement WebSocket Server with Compression**  
13. **Create Custom Protocol with Length-prefixed Frames**  
14. **Build Proxy Server with TLS Termination**  
15. **Implement Connection Multiplexing**

---

## Detailed Task Descriptions

### Binary Search Trees (BST)

#### 1. Implement BST Insertion

- **Objective**: Build the foundation for a binary search tree.  
- **Implementation**:  
  - Define a `Node` struct with `value`, `left`, and `right` fields.  
  - Create an `insert` method to add nodes recursively while maintaining BST properties.

---

#### 2. Add BST Search

- **Objective**: Implement search functionality to locate values in the BST.  
- **Implementation**:  
  - Use recursive or iterative traversal to find a node with the target value.

---

#### 3. Implement BST Deletion

- **Objective**: Enable removal of nodes while preserving BST properties.  
- **Implementation**:  
  - Handle three cases: leaf node, node with one child, and node with two children (replace with successor or predecessor).

---

#### 4. Find BST Successor

- **Objective**: Determine the in-order successor of a given node.  
- **Implementation**:  
  - If the node has a right subtree, find the minimum value in that subtree.  
  - Otherwise, backtrack to find the first ancestor where the node lies in its left subtree.

---

#### 5. Validate BST Properties

- **Objective**: Verify if a binary tree satisfies BST constraints.  
- **Implementation**:  
  - Use in-order traversal to ensure node values are in ascending order.

---

#### 6. Implement AVL Tree Rotations

- **Objective**: Introduce self-balancing to the BST with AVL rotations.  
- **Implementation**:  
  - Implement single (left and right) and double (left-right and right-left) rotations to maintain balance.

---

#### 7. Add Balancing to BST

- **Objective**: Automatically balance the BST during insertion and deletion.  
- **Implementation**:  
  - Use height differences to trigger AVL rotations and maintain balance.

---

### Network Programming

#### 8. Build Async TCP Server with Custom Protocol

- **Objective**: Implement an asynchronous TCP server that handles custom client requests.  
- **Implementation**:  
  - Use libraries like `tokio` to manage connections and process protocol-specific data.

---

#### 9. Implement HTTP/2 Server with Streaming

- **Objective**: Build an HTTP/2 server supporting bi-directional streaming.  
- **Implementation**:  
  - Use an HTTP/2 library like `hyper` to handle streams and multiplexing.

---

#### 10. Create QUIC Protocol Implementation

- **Objective**: Implement a QUIC server for low-latency, secure communication.  
- **Implementation**:  
  - Use Rust libraries like `quinn` to build the protocol.

---

#### 11. Build gRPC Service with Bidirectional Streaming

- **Objective**: Create a gRPC service supporting two-way communication.  
- **Implementation**:  
  - Use the `tonic` library to define protobufs and implement the service.

---

#### 12. Implement WebSocket Server with Compression

- **Objective**: Build a WebSocket server with message compression.  
- **Implementation**:  
  - Use libraries like `tokio-tungstenite` to handle WebSocket communication.

---

#### 13. Create Custom Protocol with Length-prefixed Frames

- **Objective**: Design a protocol using length-prefixed frames for structured communication.  
- **Implementation**:  
  - Include message size as a prefix for parsing complete frames.

---

#### 14. Build Proxy Server with TLS Termination

- **Objective**: Develop a proxy server that handles TLS termination.  
- **Implementation**:  
  - Use a library like `rustls` to decrypt incoming TLS traffic and forward plaintext data.

---

#### 15. Implement Connection Multiplexing

- **Objective**: Manage multiple connections over a single transport layer.  
- **Implementation**:  
  - Use tools like `tokio` to multiplex connections efficiently.
