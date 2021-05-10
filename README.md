Simple Transaction Processor
---

This is a small demo project for handling various types of financial transactions.

For information about running the tool use:
`cargo run -- -h`

Testing is primarily accomplished via unit-tests. This allows each state transition of importance to be verified quickly without 
too much setup. The project is organized using hexagonal "ports and adapters" architecture to improve maintainability, 
testability, and flexibility to expand in the future while keeping the core business rules intact. For example, it 
wouldn't require any modifications to the domain in order to allow this engine to process web requests and
persist results to a database if needed since it's very generic and async compatible.