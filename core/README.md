# Kasane Core

**Kasane Core** is the database implementation of Kasane, providing efficient storage and management of 4-dimensional spatio-temporal data. It serves as the foundational layer for managing drone flight paths and other spatio-temporal information with transaction support and type safety.

[🇯🇵 日本語版](./README_JA.md)

## 🌱 Features

- **4D Data Storage**: Efficient storage and retrieval of spatio-temporal data using space-time IDs
- **Transaction Support**: ACID-compliant transactions ensuring data consistency and safety
- **Type Safety**: Strong typing system preventing data corruption and ensuring reliability
- **Performance Optimized**: Designed for high-performance operations on large spatio-temporal datasets
- **WebAssembly Ready**: Can be compiled to WebAssembly for browser and edge deployment

## 🏗️ Architecture

The Kasane Core database is built with the following components:

### Storage Engine
- Efficient indexing and storage of space-time IDs
- Optimized for range queries and spatial operations
- Transaction logging and recovery mechanisms

### Parser
- Command parsing and validation
- SQL-like syntax for spatio-temporal queries
- Type checking and error handling

### Transaction Manager
- ACID transaction support
- Concurrent access control
- Rollback and recovery capabilities

## 🚀 Getting Started

### Building

```bash
# Build the core database
cargo build --package kasane

# Run the database
cargo run --package kasane
```

### Testing

```bash
# Run all tests
cargo test --package kasane

# Run with verbose output
cargo test --package kasane -- --nocapture
```

### Benchmarking

```bash
# Run performance benchmarks
cargo bench --package kasane
```

## 🔧 Usage

The Kasane Core database provides a command-line interface for managing spatio-temporal data:

```bash
# Start the database
./target/release/kasane

# Example commands (in the database CLI):
ADD_SPACE space_name;
ADD_KEYS key1 key2 key3;
PUT_VALUE space_name key1 "value1";
```

## 📊 Commands Reference

### Space Management
- `ADD_SPACE <name>` - Create a new data space
- `DELETE_SPACE <name>` - Remove a data space

### Key Management  
- `ADD_KEYS <key1> <key2> ...` - Add keys to the current space
- `DELETE_KEYS <key1> <key2> ...` - Remove keys from the current space

### Value Operations
- `PUT_VALUE <space> <key> <value>` - Insert or update a value
- `SET_VALUE <space> <key> <value>` - Set a value (create if not exists)
- `DELETE_VALUE <space> <key>` - Remove a value

### Transactions
- `TRANSACTION { <commands> }` - Execute commands in a transaction

## 🤝 Contributing

We welcome contributions to improve the Kasane Core database:

- Bug reports and feature requests
- Performance optimizations
- Documentation improvements
- Test coverage expansion

## 🧪 Testing Philosophy

The core database prioritizes data integrity and performance:

- Comprehensive unit tests for all components
- Integration tests for complex operations
- Performance benchmarks for critical paths
- Property-based testing for data consistency

Execute tests with:
```bash
cargo test --package kasane
```

## ⚡ Performance

Performance benchmarks ensure optimal operation:

- Storage and retrieval benchmarks
- Transaction performance tests
- Concurrent access benchmarks
- Memory usage profiling

Execute benchmarks with:
```bash
cargo bench --package kasane
```

## 📜 License

This core database implementation is published under the MIT License.
See the [LICENSE](../LICENSE) file for details.