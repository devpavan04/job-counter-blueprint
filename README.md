# Job Counter Blueprint

A Tangle Blueprint that tracks and logs every job it processes, maintaining persistent statistics across restarts.

## Features

- **Job Counting**: Tracks total number of jobs processed
- **Per-Job Statistics**: Maintains counts for each job type
- **Persistent Storage**: Saves statistics to disk (survives restarts)
- **Detailed Logging**: Logs each job with description and current counts
- **Statistics Retrieval**: Query current statistics via job
- **Counter Reset**: Reset all counters via job

## Jobs

### 1. Process Data (Job ID: 0)
Processes arbitrary string data and increments the job counter.

**Input**: String data to process
**Output**: Confirmation message with total job count

### 2. Get Stats (Job ID: 1)
Retrieves current job processing statistics.

**Input**: None
**Output**: JSON formatted statistics

### 3. Reset Counter (Job ID: 2)
Resets all job counters to zero.

**Input**: None
**Output**: Confirmation message

## Architecture

```
job-counter-bin/     # Binary crate - runner only
├── src/main.rs      # Blueprint runner setup

job-counter-lib/     # Library crate - core logic
├── src/
│   ├── lib.rs       # Module exports
│   ├── context.rs   # Blueprint context with storage
│   ├── storage.rs   # Persistent job statistics
│   ├── error.rs     # Error types
│   └── jobs/        # Job handlers
│       ├── mod.rs
│       ├── process_data.rs
│       ├── get_stats.rs
│       └── reset_counter.rs
```

## Storage

Statistics are persisted to `{data_dir}/job_stats.json` with the following structure:

```json
{
  "total_jobs": 42,
  "job_counts_by_id": {
    "0": 25,
    "1": 10,
    "2": 7
  },
  "last_processed": "Processing data: 'example'"
}
```

## Building and Running

```bash
# Build the blueprint
cargo build --release

# Run tests
cargo test

# Run the blueprint (requires Tangle network connection)
cargo run --bin job-counter-bin
```

## Testing

Each job includes unit tests using `TangleTestHarness`. Run tests with:

```bash
cargo test
```

## Logging

The blueprint uses structured logging via `tracing`. Each job execution is logged with:
- Job ID and description
- Current total job count
- Per-job-type counts
- Processing details

Example log output:
```
INFO Job processed! Total: 15, Job ID 0: 8 times, Description: Processing data: 'user input'
```
