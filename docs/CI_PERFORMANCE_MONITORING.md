# CI Performance Monitoring

This document describes how test performance metrics are collected and monitored in the GitHub Actions CI pipeline.

## Overview

The CI pipeline automatically:
1. **Collects** test execution metrics during every test run
2. **Analyzes** results against performance budgets
3. **Alerts** on performance regressions
4. **Reports** metrics in PR comments (for pull requests)
5. **Archives** metrics for trend tracking (on main branch)

## How It Works

### Test Execution & Metric Collection

When tests run in the `test` job:

```yaml
- name: Run unit tests
  run: cargo test --lib --verbose 2>&1 | tee test-output-unit.log

- name: Run integration tests
  run: cargo test --test '*' --verbose 2>&1 | tee test-output-integration.log

- name: Collect test metrics
  run: python3 .github/scripts/collect_test_metrics.py test-output-combined.log .github/test-metrics/current.json
```

The output is piped to log files, then parsed by `collect_test_metrics.py` to extract:
- Test counts by category (unit, integration, e2e)
- Execution times for each category
- Pass/fail results
- Regression analysis against budgets

### Performance Budget Checking

The metrics collector (`collect_test_metrics.py`) reads `.github/performance-budget.json` and:

- **Compares** actual test times against budgets
- **Calculates** percentage over budget
- **Generates** alerts for regressions
- **Classifies** as warning (15% over) or error (25% over)

Current budgets:
| Category | Budget | Alert @15% | Error @25% |
|----------|--------|-----------|-----------|
| Unit Tests | 1400ms | 1610ms | 1750ms |
| Integration | 900ms | 1035ms | 1125ms |
| E2E Tests | 720ms | 828ms | 900ms |
| Full Suite | 3000ms | 3450ms | 3750ms |

### PR Comments

For pull requests, the `generate_pr_comment.py` script creates a summary:

```
## 📊 Test Performance Report

**Measured:** 2025-01-15 12:34:56 UTC

### Performance Summary

| Category | Tests | Passed | Time (ms) | Avg/Test |
|----------|-------|--------|----------|----------|
| Unit Tests | 93 | 93 | 980 | 10.5 |
| Integration | 9 | 9 | 690 | 76.7 |
| E2E Tests | 12 | 12 | 540 | 45.0 |
| **Total** | **114** | **114** | **2210** | **19.4** |

### Status

✅ All performance checks passed!
```

The comment includes:
- Test execution summary table
- Performance status (alerts if any)
- Budget comparison details
- Links to baseline and optimization guide

### Metric Storage & Trends

On the main branch, metrics are stored for historical tracking:

```
.github/test-metrics/
├── metrics-20250115-120000.json  # From first run
├── metrics-20250115-130000.json  # From second run
└── current.json                  # Latest metrics
```

These files enable:
- Trend analysis over time
- Regression detection across commits
- Performance improvement tracking
- Historical comparisons

### Artifact Uploads

All test metrics are uploaded as GitHub Action artifacts:

```yaml
- name: Upload test metrics
  uses: actions/upload-artifact@v4
  with:
    name: test-metrics
    path: .github/test-metrics/
    retention-days: 30
```

Artifacts are retained for 30 days and accessible from the workflow run page.

## Metrics File Format

The `current.json` output contains:

```json
{
  "version": "1.0",
  "metadata": {
    "timestamp": "2025-01-15T12:34:56.789Z",
    "total_time_ms": 2210,
    "total_tests": 114,
    "total_passed": 114,
    "total_failed": 0
  },
  "metrics": {
    "unit_tests": {
      "category": "Unit Tests (src/**/*.rs #[test])",
      "count": 93,
      "passed": 93,
      "failed": 0,
      "time_ms": 980,
      "avg_per_test_ms": 10.5
    },
    "integration_tests": {
      "category": "Integration Tests (tests/integration_tests.rs)",
      "count": 9,
      "passed": 9,
      "failed": 0,
      "time_ms": 690,
      "avg_per_test_ms": 76.7
    },
    "e2e_tests": {
      "category": "E2E Tests (tests/*_e2e.rs)",
      "count": 12,
      "passed": 12,
      "failed": 0,
      "time_ms": 540,
      "avg_per_test_ms": 45.0
    },
    "all_tests": {
      "total_count": 114,
      "total_passed": 114,
      "total_failed": 0,
      "total_time_ms": 2210,
      "avg_per_test_ms": 19.4
    }
  },
  "regression_check": {
    "alerts": [],
    "has_warnings": false,
    "has_errors": false
  }
}
```

## Scripts

### collect_test_metrics.py

**Purpose:** Parse cargo test output and collect metrics

**Usage:**
```bash
python3 .github/scripts/collect_test_metrics.py <test_output.log> <metrics_output.json>
```

**Features:**
- Parses cargo test output using regex patterns
- Extracts test counts, times, pass/fail status
- Compares against performance budgets
- Generates regression alerts
- Outputs JSON report

**Exit Codes:**
- `0`: Success (no regression errors)
- `1`: Regression errors detected (budget exceeded by >25%)

### generate_pr_comment.py

**Purpose:** Generate markdown PR comment from metrics JSON

**Usage:**
```bash
python3 .github/scripts/generate_pr_comment.py <metrics.json> <output.md>
```

**Features:**
- Creates formatted markdown summary
- Includes performance table
- Highlights alerts and warnings
- References baseline docs
- Pretty-prints for readability

## Workflow Integration

The CI workflow (`ci.yml`) runs these steps in the `test` job:

1. **Run tests** → Generate log files
2. **Collect metrics** → Parse logs, generate JSON
3. **Upload artifacts** → Store metrics for retrieval
4. **Generate comment** → Create PR summary (if pull request)
5. **Post comment** → Add comment to PR (if pull request)

## Alerts & Actions

### Warning Level (15% over budget)

Yellow warning ⚠️ indicates:
- Performance is degraded but acceptable
- Suggest optimization in code review
- Does not block merge

**Example:**
```
⚠️ Unit Tests: Current 1610ms (budget: 1400ms, +15%)
```

### Error Level (25% over budget)

Red error 🔴 indicates:
- Performance regression is significant
- Should investigate cause
- May block merge (team decision)

**Example:**
```
🔴 Full Suite: Current 3750ms (budget: 3000ms, +25%)
```

## Manual Local Runs

To test metric collection locally:

```bash
# Run tests and capture output
cargo test --lib --verbose 2>&1 | tee test-output.log

# Collect metrics
python3 .github/scripts/collect_test_metrics.py test-output.log metrics.json

# View metrics
cat metrics.json | python3 -m json.tool

# Generate PR comment
python3 .github/scripts/generate_pr_comment.py metrics.json comment.md
cat comment.md
```

## Troubleshooting

### Metrics not collected

**Problem:** `collect_test_metrics.py` fails to parse output

**Solution:**
1. Check test output format matches expected patterns
2. Verify cargo version (should be recent stable)
3. Check that `--verbose` flag is present in test command
4. Look for regex pattern mismatches in script

### PR comment not posted

**Problem:** Comment doesn't appear on PR

**Solution:**
1. Check script permissions (should be executable)
2. Verify `GITHUB_TOKEN` has `write:discussion` scope
3. Check GitHub Actions logs for errors
4. Ensure `github.event_name == 'pull_request'` condition

### Budget values seem wrong

**Problem:** Alerts triggered incorrectly

**Solution:**
1. Review `.github/performance-budget.json` values
2. Check calculation: `budget * 1.15` for warning, `budget * 1.25` for error
3. Verify test times are being measured correctly
4. Compare against `docs/TEST_PERFORMANCE_BASELINE.md`

## Future Enhancements

Potential improvements:

1. **Historical comparison** — Show trend vs. previous commits
2. **Slow test detection** — Flag individual tests exceeding per-test budgets
3. **Flakiness tracking** — Detect test instability
4. **Performance graphs** — Visualize trends over time
5. **Email alerts** — Notify team of regressions
6. **Slack integration** — Post alerts to Slack channel
7. **Baseline updates** — Semi-automated baseline adjustment
8. **Test impact analysis** — Show which commits caused regressions

## References

- [Test Performance Baseline](./TEST_PERFORMANCE_BASELINE.md)
- [Test Optimization Checklist](./TEST_OPTIMIZATION_CHECKLIST.md)
- [Performance Budget Configuration](./.github/performance-budget.json)
- [CI Workflow](./.github/workflows/ci.yml)
- [Metrics Collection Script](./.github/scripts/collect_test_metrics.py)
- [PR Comment Generator](./.github/scripts/generate_pr_comment.py)