# Axum vs ActixWeb Performance Comparison Demo

A comprehensive comparison between two popular Rust web frameworks: Axum and ActixWeb. This project implements identical functionality in both frameworks to enable fair performance benchmarking.

## 🚀 Features

- **REST API Implementation** in both frameworks
- **GraphQL Support** with queries, mutations, and subscriptions
- **JWT Authentication** with user registration and login
- **Shopify Webhook Handling** for e-commerce integration
- **Performance Benchmarking** suite for detailed comparison
- **Shared Models** to ensure consistency between implementations

## 📁 Project Structure

```
axum-actixweb-demo/
├── shared/              # Shared utilities and models
├── axum-server/         # Axum implementation
├── actixweb-server/     # ActixWeb implementation  
├── benchmarks/          # Benchmarking application
├── Cargo.toml           # Workspace configuration
└── README.md
```

## 🛠️ Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Cargo** (comes with Rust)

## 🏃 Quick Start

### 1. Clone and Setup

```bash
git clone git@github.com:PostNZT/axum-actixweb-demo.git
cd axum-actixweb-demo
```

### 2. Run Both Servers

**Terminal 1 - Axum Server:**
```bash
cargo run --bin axum-server
```
Server will start on `http://localhost:3000`

**Terminal 2 - ActixWeb Server:**
```bash
cargo run --bin actixweb-server  
```
Server will start on `http://localhost:3001`

### 3. Run Benchmarks

**Terminal 3 - Benchmarks:**
```bash
# Run all benchmarks
cargo run --bin benchmarks all

# Or run specific benchmarks
cargo run --bin benchmarks health --concurrency 100 --requests 1000
cargo run --bin benchmarks rest --concurrency 50 --requests 500
cargo run --bin benchmarks graphql --concurrency 30 --requests 300
```

## 🔌 API Endpoints

Both servers implement identical endpoints:

### Health Check
- `GET /` - Basic health check
- `GET /health` - Detailed health status

### Authentication
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login (returns JWT token)

### Users
- `GET /api/users` - List all users
- `GET /api/users/{id}` - Get user by ID

### Products
- `GET /api/products` - List all products
- `POST /api/products` - Create new product
- `GET /api/products/{id}` - Get product by ID
- `PUT /api/products/{id}` - Update product
- `DELETE /api/products/{id}` - Delete product

### Webhooks
- `POST /api/webhooks/shopify` - Handle Shopify webhooks

### GraphQL
- `POST /graphql` - GraphQL endpoint
- `GET /graphiql` - GraphiQL playground

## 📊 GraphQL Schema

### Queries
```graphql
type Query {
  users: [User!]!
  user(id: UUID!): User
  products: [Product!]!
  product(id: UUID!): Product
}
```

### Mutations
```graphql
type Mutation {
  createProduct(input: CreateProductInput!): Product!
  updateProduct(id: UUID!, input: UpdateProductInput!): Product
  deleteProduct(id: UUID!): Boolean!
}
```

### Subscriptions
```graphql
type Subscription {
  productUpdates: Product!
}
```

## 🧪 Testing the APIs

### Register a User
```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com", 
    "password": "password"
  }'
```

### Login
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password"
  }'
```

### Create a Product
```bash
curl -X POST http://localhost:3000/api/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Awesome Product",
    "description": "A really awesome product",
    "price": 2999,
    "inventory": 50
  }'
```

### GraphQL Query
```bash
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "{ products { id name price inventory } }"
  }'
```

## 🏎️ Performance Benchmarking & Analysis

The benchmarking suite provides comprehensive performance comparison between Axum and ActixWeb across different workload scenarios.

### Benchmark Categories

#### 1. Health Check Benchmark
- **Purpose**: Tests basic server responsiveness and overhead
- **Endpoint**: `GET /health`
- **Default Config**: 100 concurrent connections, 1000 total requests
- **Measures**: Raw throughput and latency under minimal processing load

#### 2. REST API Benchmark  
- **Purpose**: Tests JSON processing and routing performance
- **Endpoint**: `POST /api/products` (product creation)
- **Default Config**: 50 concurrent connections, 500 total requests
- **Measures**: Performance with realistic JSON payloads and business logic

#### 3. GraphQL Benchmark
- **Purpose**: Tests GraphQL query processing and schema resolution
- **Endpoint**: `POST /graphql` (product queries)
- **Default Config**: 30 concurrent connections, 300 total requests
- **Measures**: Query parsing, validation, and execution performance

### Benchmark Commands

```bash
# Run comprehensive benchmark suite
cargo run --bin benchmarks all

# Individual benchmark types
cargo run --bin benchmarks health --concurrency 100 --requests 1000
cargo run --bin benchmarks rest --concurrency 50 --requests 500
cargo run --bin benchmarks graphql --concurrency 30 --requests 300

# Custom load testing
cargo run --bin benchmarks health --concurrency 200 --requests 10000
```

### Understanding Benchmark Results

The benchmark output provides a detailed comparison table with the following metrics:

| Metric | Description | Significance |
|--------|-------------|-------------|
| **Framework** | Axum vs ActixWeb | Framework being tested |
| **Endpoint** | API endpoint type | Test scenario (Health/REST/GraphQL) |
| **Total Requests** | Number of requests sent | Test scale |
| **Concurrency** | Concurrent connections | Load intensity |
| **Total Time (ms)** | Complete test duration | Overall test execution time |
| **Avg Response Time (ms)** | Mean response latency | Individual request performance |
| **Requests/Second** | Throughput measurement | Server capacity |
| **Success Rate (%)** | Successful responses | Reliability under load |

### Actual Benchmark Results

Below are real benchmark results from testing both frameworks on the same system:

#### Test Environment
- **OS**: Windows 11
- **CPU**: Local development machine  
- **Build**: Debug mode (for demonstration)
- **Test Date**: 2025-07-22

#### Health Check Benchmark Results
```
+-----------+--------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
| framework | endpoint     | total_requests | concurrency | total_time_ms | avg_response_time_ms | requests_per_second | success_rate |
+-----------+--------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
| Axum      | Health Check | 100            | 10          | 343           | 33.46                | 291.12              | 100          |
+-----------+--------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
| ActixWeb  | Health Check | 100            | 10          | 335           | 32.92                | 298.47              | 100          |
+-----------+--------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
```

#### REST API Benchmark Results  
```
+-----------+----------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
| framework | endpoint       | total_requests | concurrency | total_time_ms | avg_response_time_ms | requests_per_second | success_rate |
+-----------+----------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
| Axum      | Create Product | 50             | 5           | 329           | 32.62                | 151.96              | 100          |
+-----------+----------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
| ActixWeb  | Create Product | 50             | 5           | 330           | 32.76                | 151.24              | 100          |
+-----------+----------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
```

#### GraphQL Benchmark Results
```
+-----------+---------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
| framework | endpoint      | total_requests | concurrency | total_time_ms | avg_response_time_ms | requests_per_second | success_rate |
+-----------+---------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
| Axum      | GraphQL Query | 30             | 5           | 377           | 62.47                | 79.40               | 100          |
+-----------+---------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
| ActixWeb  | GraphQL Query | 30             | 5           | 338           | 55.27                | 88.52               | 100          |
+-----------+---------------+----------------+-------------+---------------+----------------------+---------------------+--------------+
```

#### Performance Analysis Summary

**Key Observations:**

1. **Health Check Performance**: Very close results with ActixWeb showing slightly better performance
   - ActixWeb: 298.47 RPS vs Axum: 291.12 RPS (~2.5% faster)
   - Response times are nearly identical (~33ms average)

2. **REST API Performance**: Nearly identical performance for JSON processing
   - Both frameworks handle product creation at ~151-152 RPS
   - Response times are virtually the same (~32.7ms average)

3. **GraphQL Performance**: ActixWeb shows better performance for complex queries
   - ActixWeb: 88.52 RPS vs Axum: 79.40 RPS (~11% faster)  
   - ActixWeb has lower response times: 55.27ms vs 62.47ms

**Overall Assessment:**
- Both frameworks show excellent reliability (100% success rate)
- Performance differences are relatively small for most use cases
- ActixWeb shows slight advantages in GraphQL processing
- Both frameworks are production-ready with comparable performance characteristics

**Note**: These are development machine results in debug mode. Production performance with release builds and optimized configurations would show significantly higher throughput for both frameworks.

### Performance Analysis Guidelines

#### Key Performance Indicators (KPIs)
1. **Requests per Second (RPS)** - Primary throughput metric
2. **Average Response Time** - Latency under normal load
3. **Success Rate** - Reliability and error handling
4. **Resource Utilization** - CPU and memory efficiency (monitor externally)

#### Interpreting Results
- **Higher RPS = Better throughput** - More requests handled per second
- **Lower response time = Better latency** - Faster individual request processing
- **100% success rate = Good reliability** - No failed requests under test load
- **Consistent performance across scenarios** - Framework stability

#### Performance Considerations

**Framework Characteristics:**
- **Axum**: Generally optimized for simplicity and type safety
- **ActixWeb**: Traditionally focused on raw performance and actor model

**Factors Affecting Performance:**
- **Middleware overhead** (CORS, logging, authentication)
- **JSON serialization/deserialization** (serde performance)
- **Database connection pooling** (when using persistent storage)
- **Memory allocation patterns** (tokio runtime efficiency)
- **HTTP/2 vs HTTP/1.1** support and optimization

### Running Production-Like Benchmarks

For more realistic performance testing:

```bash
# High load scenario
cargo run --bin benchmarks health --concurrency 500 --requests 50000

# Sustained load test  
cargo run --bin benchmarks rest --concurrency 100 --requests 10000

# Mixed workload simulation
cargo run --bin benchmarks all  # Multiple iterations
```

### Environment Considerations

**For Accurate Benchmarks:**
- Run servers and benchmarks on the same machine to eliminate network latency
- Ensure no other resource-intensive applications are running
- Use release builds for performance testing: `cargo build --release`
- Allow JIT warm-up by running benchmarks multiple times
- Monitor system resources (CPU, memory, network) during testing

**Benchmark Limitations:**
- Tests use mock data and in-memory operations
- Real-world performance may vary with database operations
- Network conditions affect production performance
- Cold start performance not measured in sustained tests

## 🔧 Development

### Building the Project
```bash
# Build all workspace members
cargo build

# Build specific components
cargo build --bin axum-server
cargo build --bin actixweb-server
cargo build --bin benchmarks
```

### Running Tests
```bash
cargo test
```

### Code Structure

- **shared/**: Common models, authentication, error handling, and GraphQL schema
- **axum-server/**: Axum-specific implementation with routing and handlers
- **actixweb-server/**: ActixWeb-specific implementation with equivalent functionality
- **benchmarks/**: Performance testing suite with configurable parameters

## 📈 Performance Considerations

Both implementations use:
- **Async/await** for non-blocking I/O
- **JSON serialization/deserialization** with serde
- **JWT authentication** for stateless auth
- **GraphQL** with async-graphql
- **CORS** middleware for cross-origin requests
- **Logging** with tracing for observability

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run benchmarks to ensure no performance regressions
6. Submit a pull request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🔗 Resources

- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [ActixWeb Documentation](https://actix.rs/docs/)
- [async-graphql Documentation](https://async-graphql.github.io/async-graphql/en/)
- [Tokio Documentation](https://tokio.rs/)