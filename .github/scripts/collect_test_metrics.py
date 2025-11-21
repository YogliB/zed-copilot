#!/usr/bin/env python3
"""
Test Performance Metrics Collector

Parses `cargo test` output and collects performance metrics.
Generates JSON report with test counts, execution times, and regression detection.

Usage:
    python collect_test_metrics.py <output_file> <metrics_output>

    output_file: File containing stdout from cargo test runs
    metrics_output: Output JSON file with collected metrics
"""

import json
import re
import sys
from datetime import datetime
from pathlib import Path
from typing import Any, Dict


class TestMetricsCollector:
    def __init__(self):
        self.test_results = {
            "unit": {"count": 0, "passed": 0, "failed": 0, "time_ms": 0},
            "integration": {"count": 0, "passed": 0, "failed": 0, "time_ms": 0},
            "e2e": {"count": 0, "passed": 0, "failed": 0, "time_ms": 0},
        }
        self.metadata = {
            "timestamp": datetime.utcnow().isoformat(),
            "total_time_ms": 0,
            "total_tests": 0,
            "total_passed": 0,
            "total_failed": 0,
        }

    def parse_cargo_output(self, output: str) -> None:
        """Parse cargo test output and extract metrics."""
        lines = output.split("\n")

        # Pattern to match test result summary lines
        result_pattern = r"test result: (?P<result>ok|FAILED)\. (?P<passed>\d+) passed; (?P<failed>\d+) failed.*finished in (?P<time>[\d.]+)s"

        # Split output by running tests sections
        sections = []
        current_section = []

        for line in lines:
            if "running" in line and "tests" in line:
                if current_section:
                    sections.append("\n".join(current_section))
                current_section = [line]
            else:
                current_section.append(line)

        if current_section:
            sections.append("\n".join(current_section))

        # Process each section
        categories = ["unit", "integration", "e2e"]

        for idx, section in enumerate(sections):
            # Find result line in section
            for line in section.split("\n"):
                if match := re.search(result_pattern, line):
                    passed = int(match.group("passed"))
                    failed = int(match.group("failed"))
                    time_ms = int(float(match.group("time")) * 1000)

                    # Assign category based on order of appearance
                    if idx < len(categories):
                        category = categories[idx]
                        self.test_results[category]["count"] = passed + failed
                        self.test_results[category]["passed"] = passed
                        self.test_results[category]["failed"] = failed
                        self.test_results[category]["time_ms"] = time_ms
                    break

    def calculate_aggregates(self) -> None:
        """Calculate aggregate metrics."""
        for category in self.test_results.values():
            self.metadata["total_tests"] += category["count"]
            self.metadata["total_passed"] += category["passed"]
            self.metadata["total_failed"] += category["failed"]
            self.metadata["total_time_ms"] += category["time_ms"]

    def generate_report(self) -> Dict[str, Any]:
        """Generate final metrics report."""
        return {
            "version": "1.0",
            "metadata": self.metadata,
            "metrics": {
                "unit_tests": {
                    "category": "Unit Tests (src/**/*.rs #[test])",
                    "count": self.test_results["unit"]["count"],
                    "passed": self.test_results["unit"]["passed"],
                    "failed": self.test_results["unit"]["failed"],
                    "time_ms": self.test_results["unit"]["time_ms"],
                    "avg_per_test_ms": (
                        self.test_results["unit"]["time_ms"] / max(1, self.test_results["unit"]["count"])
                    ),
                },
                "integration_tests": {
                    "category": "Integration Tests (tests/integration_tests.rs)",
                    "count": self.test_results["integration"]["count"],
                    "passed": self.test_results["integration"]["passed"],
                    "failed": self.test_results["integration"]["failed"],
                    "time_ms": self.test_results["integration"]["time_ms"],
                    "avg_per_test_ms": (
                        self.test_results["integration"]["time_ms"] / max(1, self.test_results["integration"]["count"])
                    ),
                },
                "e2e_tests": {
                    "category": "E2E Tests (tests/*_e2e.rs)",
                    "count": self.test_results["e2e"]["count"],
                    "passed": self.test_results["e2e"]["passed"],
                    "failed": self.test_results["e2e"]["failed"],
                    "time_ms": self.test_results["e2e"]["time_ms"],
                    "avg_per_test_ms": (
                        self.test_results["e2e"]["time_ms"] / max(1, self.test_results["e2e"]["count"])
                    ),
                },
                "all_tests": {
                    "total_count": self.test_results["unit"]["count"] + self.test_results["integration"]["count"] + self.test_results["e2e"]["count"],
                    "total_passed": self.test_results["unit"]["passed"] + self.test_results["integration"]["passed"] + self.test_results["e2e"]["passed"],
                    "total_failed": self.test_results["unit"]["failed"] + self.test_results["integration"]["failed"] + self.test_results["e2e"]["failed"],
                    "total_time_ms": self.test_results["unit"]["time_ms"] + self.test_results["integration"]["time_ms"] + self.test_results["e2e"]["time_ms"],
                },
            },
        }

    def finalize_report(self) -> Dict[str, Any]:
        """Finalize report with computed averages."""
        report = self.generate_report()
        all_metrics = report["metrics"]["all_tests"]
        all_metrics["avg_per_test_ms"] = (
            all_metrics["total_time_ms"] / max(1, all_metrics["total_count"])
        )
        return report

    def check_regression(self) -> Dict[str, Any]:
        """Check for performance regressions against budget."""
        alerts = []

        # Load performance budget
        budget_path = Path(__file__).parent.parent / "performance-budget.json"
        if not budget_path.exists():
            return {"alerts": alerts, "has_warnings": False, "has_errors": False}

        with open(budget_path) as f:
            budget = json.load(f)

        current = self.finalize_report()["metrics"]

        # Check unit tests
        unit_time = current["unit_tests"]["time_ms"]
        unit_budget = budget["budgets"]["unitTests"]["totalBudgetMs"]
        if unit_time > 0 and unit_time > unit_budget * 1.15:
            severity = "warning" if unit_time < unit_budget * 1.25 else "error"
            alerts.append({
                "category": "unit_tests",
                "severity": severity,
                "current_ms": unit_time,
                "budget_ms": unit_budget,
                "percent_over": round((unit_time - unit_budget) / unit_budget * 100, 1),
                "message": f"Unit tests took {unit_time}ms (budget: {unit_budget}ms, +{(unit_time - unit_budget) / unit_budget * 100:.1f}%)",
            })

        # Check integration tests
        int_time = current["integration_tests"]["time_ms"]
        int_budget = budget["budgets"]["integrationTests"]["totalBudgetMs"]
        if int_time > 0 and int_time > int_budget * 1.15:
            severity = "warning" if int_time < int_budget * 1.25 else "error"
            alerts.append({
                "category": "integration_tests",
                "severity": severity,
                "current_ms": int_time,
                "budget_ms": int_budget,
                "percent_over": round((int_time - int_budget) / int_budget * 100, 1),
                "message": f"Integration tests took {int_time}ms (budget: {int_budget}ms, +{(int_time - int_budget) / int_budget * 100:.1f}%)",
            })

        # Check E2E tests
        e2e_time = current["e2e_tests"]["time_ms"]
        e2e_budget = budget["budgets"]["e2eTests"]["totalBudgetMs"]
        if e2e_time > 0 and e2e_time > e2e_budget * 1.15:
            severity = "warning" if e2e_time < e2e_budget * 1.25 else "error"
            alerts.append({
                "category": "e2e_tests",
                "severity": severity,
                "current_ms": e2e_time,
                "budget_ms": e2e_budget,
                "percent_over": round((e2e_time - e2e_budget) / e2e_budget * 100, 1),
                "message": f"E2E tests took {e2e_time}ms (budget: {e2e_budget}ms, +{(e2e_time - e2e_budget) / e2e_budget * 100:.1f}%)",
            })

        # Check full suite
        total_time = current["all_tests"]["total_time_ms"]
        full_budget = budget["budgets"]["fullSuite"]["totalBudgetMs"]
        if total_time > 0 and total_time > full_budget * 1.15:
            severity = "warning" if total_time < full_budget * 1.25 else "error"
            alerts.append({
                "category": "full_suite",
                "severity": severity,
                "current_ms": total_time,
                "budget_ms": full_budget,
                "percent_over": round((total_time - full_budget) / full_budget * 100, 1),
                "message": f"Full test suite took {total_time}ms (budget: {full_budget}ms, +{(total_time - full_budget) / full_budget * 100:.1f}%)",
            })

        return {
            "alerts": alerts,
            "has_warnings": any(a["severity"] == "warning" for a in alerts),
            "has_errors": any(a["severity"] == "error" for a in alerts),
        }


def main():
    if len(sys.argv) < 3:
        print("Usage: python collect_test_metrics.py <cargo_output_file> <metrics_output_file>")
        sys.exit(1)

    output_file = Path(sys.argv[1])
    metrics_output = Path(sys.argv[2])

    # Read cargo output
    if not output_file.exists():
        print(f"Error: Input file not found: {output_file}")
        sys.exit(1)

    with open(output_file) as f:
        cargo_output = f.read()

    # Collect metrics
    collector = TestMetricsCollector()
    collector.parse_cargo_output(cargo_output)
    collector.calculate_aggregates()

    # Generate report
    report = collector.finalize_report()

    # Check for regressions
    regression_check = collector.check_regression()
    report["regression_check"] = regression_check

    # Write metrics
    metrics_output.parent.mkdir(parents=True, exist_ok=True)
    with open(metrics_output, "w") as f:
        json.dump(report, f, indent=2)

    print(f"✓ Metrics collected: {metrics_output}")
    print(f"  Total tests: {report['metadata']['total_tests']}")
    print(f"  Total time: {report['metadata']['total_time_ms']}ms")

    if regression_check["alerts"]:
        print("\n⚠️  Performance Alerts:")
        for alert in regression_check["alerts"]:
            print(f"  [{alert['severity'].upper()}] {alert['message']}")

    sys.exit(1 if regression_check["has_errors"] else 0)


if __name__ == "__main__":
    main()
