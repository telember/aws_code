# AWS Security Group Scanner

Rust tool to identify unused AWS security groups. Scans EC2 instances and Network Interfaces (ENIs) to find security groups that aren't actively being used.

## Quick Start

1. Ensure AWS credentials are configured
2. Run:
```bash
cargo run
```

## Requirements
- Rust 2021+
- AWS credentials with EC2 describe permissions

The tool will display a table of unused security groups showing their IDs, names, and VPC IDs.
