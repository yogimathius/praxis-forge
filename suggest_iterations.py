import json

# Define component priorities (earlier = higher priority)
component_order = {
    "Goal Tracking": 1,
    "Task Management": 2,
    "UI Components": 5,
    "Database": 6,
    "State management": 7,
    "Tauri": 8,
    "Frontend": 9,
    "Innovations": 10,
    "Testing": 11,
    "Tooling": 12,
}


def suggest_iteration(item):
    component = item["component"]
    title = title_lower = item["title"].lower()

    # Testing-related items go to later iterations
    if "test" in title_lower or component == "Testing":
        return "Q2C4 2025"

    # Task Management handling
    if component == "Task Management":
        if "setup" in title_lower or "basic" in title_lower or "core" in title_lower:
            return "Q1C4 2025"
        return "Q1C5 2025"

    # Goal Tracking handling
    if component == "Goal Tracking":
        if "setup" in title_lower or "basic" in title_lower or "core" in title_lower:
            return "Q1C2 2025"
        return "Q1C3 2025"

    # Database and State Management
    if component in ["Database", "State management"]:
        if "backup" in title_lower or "migration" in title_lower:
            return "Q1C4 2025"
        return "Q1C5 2025"

    # Frontend and UI Components distribution
    if component in ["Frontend", "UI Components"]:
        if "animation" in title_lower or "style" in title_lower:
            return "Q1C5 2025"
        if "accessibility" in title_lower or "loading" in title_lower:
            return "Q1C4 2025"
        return "Q1C3 2025"

    # Tauri setup and configuration
    if component == "Tauri":
        if "package" in title_lower or "update" in title_lower:
            return "Q1C5 2025"
        return "Q1C4 2025"

    # Innovations distribution
    if component == "Innovations":
        # Core improvements (Q2C1)
        if any(
            keyword in title_lower
            for keyword in ["error message", "documentation", "boilerplate"]
        ):
            return "Q1C6 2025"
        # Build and setup (Q2C2)
        if any(keyword in title_lower for keyword in ["scss", "build", "setup"]):
            return "Q1C5 2025"
        # Performance and optimization (Q2C3)
        if any(
            keyword in title_lower
            for keyword in ["optimize", "performance", "binary", "profiling"]
        ):
            return "Q1C6 2025"
        # Testing patterns (Q2C4)
        if "test" in title_lower:
            return "Q2C1 2025"
        # Native features (Q2C5)
        if any(keyword in title_lower for keyword in ["native", "rust-js"]):
            return "Q2C2 2025"
        # Everything else
        return "Q2C1 2025"

    # Tooling goes to final iteration
    if component == "Tooling":
        return "Q2C5 2025"

    # Default mapping for anything else
    base_priority = component_order.get(component, 99)
    if base_priority <= 6:
        return "Q2C1 2025"
    elif base_priority <= 8:
        return "Q2C2 2025"
    elif base_priority <= 10:
        return "Q2C3 2025"
    else:
        return "Q2C5 2025"


# Read current issues
with open("issues.json", "r") as f:
    issues = json.load(f)

# Update iterations for items with "No Status"
updated_issues = []
for issue in issues:
    if issue["status"] == "No Status":
        issue["iteration"] = suggest_iteration(issue)
    updated_issues.append(issue)

# Sort by iteration
updated_issues.sort(key=lambda x: x["iteration"])

# Save updated issues
with open("updated_issues.json", "w") as f:
    json.dump(updated_issues, f, indent=2)

# Print summary of changes
print("\nSuggested Iterations:")
for iteration in sorted(set(issue["iteration"] for issue in updated_issues)):
    issues_in_iteration = [i for i in updated_issues if i["iteration"] == iteration]
    print(f"\n{iteration}:")
    for issue in issues_in_iteration:
        print(f"- {issue['title']} ({issue['component']})")
