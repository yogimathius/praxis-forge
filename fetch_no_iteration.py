#!/usr/bin/env python3
import requests
import json
import os

# Configuration
GITHUB_TOKEN = "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"
OWNER = "yogimathius"
PROJECT_NUMBER = 11


def fetch_project_items(cursor=None):
    """Fetch items from GitHub Project v2"""
    query = """
    query($owner: String!, $number: Int!, $cursor: String) {
      user(login: $owner) {
        projectV2(number: $number) {
          items(first: 100, after: $cursor) {
            pageInfo {
              hasNextPage
              endCursor
            }
            nodes {
              id
              fieldValues(first: 20) {
                nodes {
                  ... on ProjectV2ItemFieldTextValue {
                    text
                    field { ... on ProjectV2FieldCommon { name } }
                  }
                  ... on ProjectV2ItemFieldSingleSelectValue {
                    name
                    field { ... on ProjectV2FieldCommon { name } }
                  }
                  ... on ProjectV2ItemFieldIterationValue {
                    title
                    field { ... on ProjectV2FieldCommon { name } }
                  }
                }
              }
            }
          }
        }
      }
    }
    """

    variables = {"owner": OWNER, "number": PROJECT_NUMBER, "cursor": cursor}

    response = requests.post(
        "https://api.github.com/graphql",
        json={"query": query, "variables": variables},
        headers={"Authorization": f"Bearer {GITHUB_TOKEN}"},
    )

    return response.json()


def process_items(items):
    """Process items and extract relevant information"""
    processed_items = []
    for item in items:
        field_values = item["fieldValues"]["nodes"]
        processed_item = {
            "id": item["id"],
            "title": next(
                (
                    fv["text"]
                    for fv in field_values
                    if fv.get("field", {}).get("name") == "Title"
                ),
                None,
            ),
            "status": next(
                (
                    fv["name"]
                    for fv in field_values
                    if fv.get("field", {}).get("name") == "Status"
                ),
                "No Status",
            ),
            "component": next(
                (
                    fv["name"]
                    for fv in field_values
                    if fv.get("field", {}).get("name") == "Component"
                ),
                "Uncategorized",
            ),
            "iteration": next(
                (
                    fv["title"]
                    for fv in field_values
                    if fv.get("field", {}).get("name") == "Iteration"
                ),
                "No Iteration",
            ),
        }
        processed_items.append(processed_item)
    return processed_items


def main():
    all_items = []
    cursor = None

    while True:
        print(f"Fetching items{' after cursor ' + cursor if cursor else ''}...")
        response = fetch_project_items(cursor)

        if "errors" in response:
            print("Error fetching data:", response["errors"])
            break

        items = response["data"]["user"]["projectV2"]["items"]["nodes"]
        page_info = response["data"]["user"]["projectV2"]["items"]["pageInfo"]

        processed_items = process_items(items)
        all_items.extend(processed_items)

        if not page_info["hasNextPage"]:
            break

        cursor = page_info["endCursor"]

    # Filter for items with no iteration and not done/frozen
    no_iteration_items = [
        item
        for item in all_items
        if item["iteration"] == "No Iteration"
        and item["status"] not in ["Done", "Frozen"]
    ]

    # Save to file
    with open("no_iteration_issues.json", "w") as f:
        json.dump(no_iteration_items, f, indent=2)

    # Print summary
    print(f"\nFound {len(no_iteration_items)} issues with no iteration:")
    for item in no_iteration_items:
        print(f"- {item['title']} ({item['component']})")
    print(f"\nTotal items fetched: {len(all_items)}")
    print(f"Items with no iteration: {len(no_iteration_items)}")
    print("Results saved to no_iteration_issues.json")


if __name__ == "__main__":
    main()
