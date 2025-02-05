#!/usr/bin/env python3
import json


def determine_priority(item):
    """
    Determine priority (P0, P1, P2) based on the action/dependency within its iteration
    P0: Foundation/setup tasks for the iteration
    P1: Main feature implementation
    P2: Enhancement/polish tasks
    """
    title_lower = item["title"].lower()

    # P0: Foundation/setup tasks
    if any(
        action in title_lower
        for action in [
            "setup",
            "initialize",
            "configure",
            "create base",
            "implement basic",
            "core",
            "foundation",
        ]
    ):
        return "P0"

    # P2: Enhancement/polish tasks
    if any(
        action in title_lower
        for action in [
            "enhance",
            "improve",
            "optimize",
            "polish",
            "refine",
            "add style",
            "animation",
            "advanced",
        ]
    ):
        return "P2"

    # P1: Main implementation tasks (default)
    return "P1"


def main():
    # Read the updated issues
    with open("updated_issues.json", "r") as f:
        issues = json.load(f)

    # Add priorities
    prioritized_issues = []
    for issue in issues:
        issue["priority"] = determine_priority(issue)
        prioritized_issues.append(issue)

    # Sort by iteration and then priority
    prioritized_issues.sort(key=lambda x: (x["iteration"], x["priority"]))

    # Save updated issues
    with open("prioritized_issues.json", "w") as f:
        json.dump(prioritized_issues, f, indent=2)

    # Print summary
    print("\nPrioritized Issues by Iteration:")
    current_iteration = None
    for issue in prioritized_issues:
        if issue["iteration"] != current_iteration:
            current_iteration = issue["iteration"]
            print(f"\n{current_iteration}:")
        print(f"[{issue['priority']}] {issue['title']} ({issue['component']})")

    # Print statistics per iteration
    print("\nPriority Distribution by Iteration:")
    iterations = sorted(set(issue["iteration"] for issue in prioritized_issues))
    for iteration in iterations:
        iteration_issues = [
            i for i in prioritized_issues if i["iteration"] == iteration
        ]
        p0_count = sum(1 for i in iteration_issues if i["priority"] == "P0")
        p1_count = sum(1 for i in iteration_issues if i["priority"] == "P1")
        p2_count = sum(1 for i in iteration_issues if i["priority"] == "P2")
        print(f"\n{iteration}:")
        print(f"P0 (Setup/Foundation): {p0_count}")
        print(f"P1 (Implementation): {p1_count}")
        print(f"P2 (Enhancement): {p2_count}")


if __name__ == "__main__":
    main()
