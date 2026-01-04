# Backend Integration Tests

This directory contains comprehensive integration tests for the My-Invest Backend API.

## Test Structure

```
tests/
├── common/
│   ├── mod.rs           # Common test utilities and fixtures
│   └── test_app.rs      # Test application setup with testcontainers
├── unit/
│   └── ...              # Unit tests (run with cargo test --lib)
└── integration/
    └── api/
        ├── auth_test.rs       # Authentication endpoint tests (19 tests)
        ├── asset_test.rs      # Asset endpoint tests (15 tests)
        └── watchlist_test.rs  # Watchlist endpoint tests (23 tests)
```

## Test Coverage

### Authentication Tests (19 tests)
- **Registration**:
  - Success with valid data
  - Duplicate email rejection
  - Invalid email validation
  - Password strength requirements (length, letters, numbers)
  - Password mismatch detection
  - Missing field validation

- **Login**:
  - Success with valid credentials
  - Invalid credentials rejection
  - Nonexistent user handling

- **Token Management**:
  - Refresh token success
  - Invalid refresh token rejection
  - Logout functionality
  - Token revocation after logout

- **Protected Routes**:
  - Access without token (401)
  - Access with invalid token (401)
  - Get current user info with valid token

### Asset Tests (15 tests)
- **Search**:
  - Successful search with results
  - Empty query validation
  - Missing query parameter handling
  - Unauthorized access rejection

- **Quote Retrieval**:
  - Successful quote fetching
  - Invalid symbol handling
  - Unauthorized access rejection

- **Historical Data**:
  - Success for all timeframes (1D, 1W, 1M, 1Y)
  - Invalid timeframe rejection
  - Missing timeframe parameter handling
  - Unauthorized access rejection

- **Advanced Features**:
  - Caching behavior verification
  - Symbol case insensitivity
  - Search result relevance
  - Concurrent request handling

### Watchlist Tests (23 tests)
- **CRUD Operations**:
  - Create watchlist with valid data
  - Empty/invalid name validation
  - Name length validation (max 50 chars)
  - Duplicate name rejection
  - List all watchlists
  - Empty list handling
  - Get specific watchlist
  - Not found handling
  - Invalid ObjectId format handling
  - Update watchlist name
  - Delete watchlist

- **Asset Management**:
  - Add asset to watchlist
  - Duplicate asset rejection
  - Invalid symbol handling
  - Remove asset from watchlist
  - Remove non-existent asset handling
  - Multiple assets in watchlist

- **Authorization & Isolation**:
  - User isolation (users can't access others' watchlists)
  - Unauthorized access rejection

- **Edge Cases**:
  - Asset limit enforcement
  - Timestamp tracking (created_at, updated_at)
  - Asset added_at timestamps
  - Comprehensive end-to-end workflow

## Total Test Count
- **Unit Tests**: 62 (all passing)
- **Integration Tests**: 57 (auth: 19, asset: 15, watchlist: 23)
- **Total**: 119 tests

## Prerequisites

### Required Services
1. **Docker**: For running test containers
2. **MongoDB**: Automatically started via testcontainers
3. **Redis**: Automatically started via testcontainers

### Installation

Ensure Docker is installed and running:
```bash
# Check Docker installation
docker --version

# Check Docker is running
docker ps
```

For WSL 2 users:
- Install Docker Desktop for Windows
- Enable WSL 2 integration in Docker Desktop settings
- Restart WSL distribution

## Running Tests

### Run All Tests
```bash
# From backend directory
cargo test
```

### Run Only Unit Tests
```bash
cargo test --lib
```

### Run Only Integration Tests
```bash
cargo test --test '*'
```

### Run Specific Test Suite
```bash
# Authentication tests only
cargo test --test auth_test

# Asset tests only
cargo test --test asset_test

# Watchlist tests only
cargo test --test watchlist_test
```

### Run Specific Test
```bash
cargo test test_register_success
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Tests with Logging
```bash
RUST_LOG=debug cargo test
```

## Test Execution Notes

### Serialization
Integration tests use `#[serial]` annotation to run sequentially. This is necessary because:
- Tests share Docker testcontainers
- Each test creates isolated database instances
- Prevents resource conflicts and port collisions

### Test Isolation
Each test:
- Creates a fresh `TestApp` instance
- Uses isolated MongoDB and Redis containers
- Registers unique test users (unique emails)
- Cleans up automatically after completion

### Performance
- First test run downloads container images (slower)
- Subsequent runs reuse images (faster)
- Typical execution time: 2-5 seconds per test
- Full suite: ~3-5 minutes

## Test Infrastructure

### TestApp Helper
The `TestApp` struct provides:
- Automatic container lifecycle management
- Mock Alpha Vantage server setup
- Pre-configured test server
- Helper methods for common operations:
  - `register_user(email, password)` - Register and get tokens
  - `login_user(email, password)` - Login and get tokens
  - `create_watchlist(token, name)` - Create a watchlist

### Mock Services
- **Alpha Vantage API**: Mocked with wiremock
  - Symbol search responses
  - Quote data
  - Historical data (daily timeframe)

### Test Data
- Users: Unique emails per test
- Passwords: All meet strength requirements (8+ chars, letters, numbers)
- Symbols: Primarily "AAPL" (mocked in Alpha Vantage)

## CI/CD Integration

### GitHub Actions Example
```yaml
name: Backend Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      mongodb:
        image: mongo:latest
        ports:
          - 27017:27017

      redis:
        image: redis:latest
        ports:
          - 6379:6379

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable

      - name: Run tests
        run: |
          cd backend
          cargo test --verbose
```

## Troubleshooting

### Docker Not Found
```
Error: Docker not available
```
**Solution**: Install Docker Desktop and enable WSL 2 integration

### Port Already in Use
```
Error: Address already in use (os error 98)
```
**Solution**: Stop other MongoDB/Redis instances or change test ports

### Slow Test Execution
```
Tests taking > 10 minutes
```
**Solution**:
- First run downloads images (expected)
- Check Docker resource limits
- Run with `--test-threads=1` to reduce contention

### Test Failures
```
assertion failed: response.status_code() == 200
```
**Solution**:
- Check test logs: `cargo test -- --nocapture`
- Verify containers are running: `docker ps`
- Check MongoDB/Redis connectivity

## Best Practices

1. **Test Independence**: Each test must be independent
2. **Unique Data**: Use unique emails/names to avoid conflicts
3. **Cleanup**: Tests clean up automatically via Drop
4. **Assertions**: Use descriptive assertion messages
5. **Error Handling**: Tests should fail with clear error messages

## Future Enhancements

- [ ] Add performance benchmarks
- [ ] Add load testing scenarios
- [ ] Test rate limiting behavior
- [ ] Test WebSocket connections (Phase 2)
- [ ] Add mutation testing
- [ ] Implement snapshot testing for API responses
- [ ] Add security penetration tests
- [ ] Test database migration rollbacks

## Code Coverage

To generate coverage reports:

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out Html --output-dir coverage
```

Open `coverage/index.html` to view the report.

## Contributing

When adding new tests:
1. Follow AAA pattern (Arrange, Act, Assert)
2. Use descriptive test names: `test_<operation>_<scenario>`
3. Add tests to appropriate file (auth/asset/watchlist)
4. Ensure tests are independent and idempotent
5. Update this README with new test counts

## Support

For issues or questions:
- Check existing tests for examples
- Review common test utilities in `tests/common/`
- Refer to [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
