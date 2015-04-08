# orion-backend
Backend for Orion

[![Build Status](https://travis-ci.org/samdolt/orion-backend.svg?branch=master)](https://travis-ci.org/samdolt/orion-backend)

```bash
cargo test --no-run
kcov --exclude-pattern=/.cargo target/cov target/debug/orion_logger-aa8ba5ecad312940
xdg-open target/cov/html
```

See http://users.rust-lang.org/t/tutorial-how-to-collect-test-coverages-for-rust-project/650/2
