# Task: Implement a Ring Buffer in Rust

## Overview

A ring buffer is a fixed-size, circular data structure where new elements overwrite the oldest ones when the buffer is full. This project implements a generic ring buffer in Rust with support for constant-time operations.

## Features

1. **Push:** Adds an element, overwriting the oldest if the buffer is full.
2. **Pop:** Removes and returns the oldest element.
3. **Peek:** Views the oldest element without removing it.
4. **Size/Capacity Checks:** Efficiently tracks buffer size, fullness, and emptiness.
5. **Iteration:** Allows traversal of the buffer's contents.

## Challenges

Index Management: Ensuring correct wrapping of indices to maintain circular behavior.
Generic Initialization: Avoiding reliance on trait bounds like Clone or Default by using iterator-based initialization.
Edge Cases: Correctly handling full, empty, and overflow states.

## Key Insights

Efficiency: Push and pop operations execute in O(1) time, maintaining performance irrespective of size.
Safety: Preventing out-of-bounds errors through modulo arithmetic for index wrapping.
Robustness: The implementation supports a variety of generic types without unnecessary constraints.

## Lessons Learned

Properly managing indices and capacity is critical to maintaining the integrity of the ring buffer.
Iterator-based initialization avoids unnecessary trait bounds and improves flexibility.
Edge cases like buffer overwrites and empty states require rigorous testing.

## Conclusion

This project highlights the utility and efficiency of ring buffers in scenarios requiring constant-time data access and fixed memory usage.
