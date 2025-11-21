#!/usr/bin/env python3
"""
GitHub PR Comment Generator

Generates markdown comments for GitHub PRs with test performance metrics.
Posts performance alerts and regression information.

Usage:
    python generate_pr_comment.py <metrics_file> <output_file>

    metrics_file: JSON file with test metrics (from collect_test_metrics.py)
    output_file: Output markdown file for PR comment
"""

import json
import sys
from datetime import datetime
from pathlib import Path
from typing import Any, Dict


class PRCommentGenerator:
    def __init__(self, metrics: Dict[str, Any]):
        self.metrics = metrics
        self.regression = metrics.get("regression_check", {})
        self.alerts = self.regression.get("alerts", [])

    def generate_summary_table(self) -> str:
        """Generate performance summary table."""
        m = self.metrics["metrics"]

        table = """| Category | Tests | Passed | Time (ms) | Avg/Test |
|----------|-------|--------|----------|----------|
"""

        # Unit tests
        unit = m["unit_tests"]
        table += f"| Unit Tests | {unit['count']} | {unit['passed']} | {unit['time_ms']} | {unit['avg_per_test_ms']:.1f} |\n"

        # Integration tests
        integration = m["integration_tests"]
        table += f"| Integration | {integration['count']} | {integration['passed']} | {integration['time_ms']} | {integration['avg_per_test_ms']:.1f} |\n"

        # E2E tests
        e2e = m["e2e_tests"]
        table += f"| E2E Tests | {e2e['count']} | {e2e['passed']} | {e2e['time_ms']} | {e2e['avg_per_test_ms']:.1f} |\n"

        # Total
        total = m["all_tests"]
        table += f"| **Total** | **{total['total_count']}** | **{total['total_passed']}** | **{total['total_time_ms']}** | **{total['avg_per_test_ms']:.1f}** |\n"

        return table

    def generate_alerts_section(self) -> str:
        """Generate alerts section if there are regressions."""
        if not self.alerts:
            return "✅ All performance checks passed!\n"

        section = ""
        has_errors = any(a["severity"] == "error" for a in self.alerts)

        if has_errors:
            section += "⛔ **Performance Regressions Detected**\n\n"
        else:
            section += "⚠️ **Performance Warnings**\n\n"

        for alert in self.alerts:
            emoji = "🔴" if alert["severity"] == "error" else "🟡"
            section += f"{emoji} **{alert['category'].replace('_', ' ').title()}**\n"
            section += f"   - Current: {alert['current_ms']}ms\n"
            section += f"   - Budget: {alert['budget_ms']}ms\n"
            section += f"   - Over: +{alert['percent_over']}%\n\n"

        return section

    def generate_comment(self) -> str:
        """Generate full PR comment."""
        timestamp = datetime.fromisoformat(self.metrics["metadata"]["timestamp"]).strftime("%Y-%m-%d %H:%M:%S UTC")

        comment = f"""## 📊 Test Performance Report

**Measured:** {timestamp}

### Performance Summary

{self.generate_summary_table()}

### Status

{self.generate_alerts_section()}

### Details

- **Total Tests:** {self.metrics['metadata']['total_tests']}
- **Passed:** {self.metrics['metadata']['total_passed']}
- **Failed:** {self.metrics['metadata']['total_failed']}
- **Total Time:** {self.metrics['metadata']['total_time_ms']}ms

---

<details>
<summary>📈 Performance Budgets</summary>

| Category | Budget (ms) | Current (ms) | Status |
|----------|----------|----------|--------|
"""

        unit = self.metrics["metrics"]["unit_tests"]
        unit_status = "✅" if unit["time_ms"] <= 1400 else "⚠️" if unit["time_ms"] <= 1610 else "❌"
        comment += f"| Unit Tests | 1400 | {unit['time_ms']} | {unit_status} |\n"

        integration = self.metrics["metrics"]["integration_tests"]
        int_status = "✅" if integration["time_ms"] <= 900 else "⚠️" if integration["time_ms"] <= 1035 else "❌"
        comment += f"| Integration | 900 | {integration['time_ms']} | {int_status} |\n"

        e2e = self.metrics["metrics"]["e2e_tests"]
        e2e_status = "✅" if e2e["time_ms"] <= 720 else "⚠️" if e2e["time_ms"] <= 828 else "❌"
        comment += f"| E2E | 720 | {e2e['time_ms']} | {e2e_status} |\n"

        total = self.metrics["metrics"]["all_tests"]
        total_status = "✅" if total["total_time_ms"] <= 3000 else "⚠️" if total["total_time_ms"] <= 3450 else "❌"
        comment += f"| Full Suite | 3000 | {total['total_time_ms']} | {total_status} |\n"

        comment += "\n</details>\n\n"

        if self.regression.get("has_errors"):
            comment += "**Action Required:** Please review performance regressions before merging.\n"
        elif self.regression.get("has_warnings"):
            comment += "**Note:** Performance warnings detected. Consider optimization opportunities.\n"
        else:
            comment += "All tests completed within performance budgets. Great work! 🎉\n"

        return comment


def main():
    if len(sys.argv) < 3:
        print("Usage: python generate_pr_comment.py <metrics_file> <output_file>")
        sys.exit(1)

    metrics_file = Path(sys.argv[1])
    output_file = Path(sys.argv[2])

    # Read metrics
    if not metrics_file.exists():
        print(f"Error: Metrics file not found: {metrics_file}")
        sys.exit(1)

    with open(metrics_file) as f:
        metrics = json.load(f)

    # Generate comment
    generator = PRCommentGenerator(metrics)
    comment = generator.generate_comment()

    # Write output
    output_file.parent.mkdir(parents=True, exist_ok=True)
    with open(output_file, "w") as f:
        f.write(comment)

    print(f"✓ PR comment generated: {output_file}")

    # Also print to stdout for debugging
    print("\nGenerated Comment:")
    print("=" * 80)
    print(comment)
    print("=" * 80)


if __name__ == "__main__":
    main()
