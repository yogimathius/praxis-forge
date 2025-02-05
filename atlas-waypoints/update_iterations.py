#!/usr/bin/env python3
import json
import requests
from time import sleep

# Configuration
GITHUB_TOKEN = "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"
OWNER = "yogimathius"
PROJECT_NUMBER = 11
GITHUB_API = "https://api.github.com/graphql"


def get_project_fields():
    """Get project fields to find the Iteration and Priority field IDs"""
    query = """
    query($owner: String!, $number: Int!) {
      user(login: $owner) {
        projectV2(number: $number) {
          fields(first: 20) {
            nodes {
              ... on ProjectV2IterationField {
                id
                name
                configuration {
                  iterations {
                    id
                    title
                  }
                }
              }
              ... on ProjectV2SingleSelectField {
                id
                name
                options {
                  id
                  name
                }
              }
            }
          }
        }
      }
    }
    """

    variables = {"owner": OWNER, "number": PROJECT_NUMBER}

    response = requests.post(
        GITHUB_API,
        json={"query": query, "variables": variables},
        headers={"Authorization": f"Bearer {GITHUB_TOKEN}"},
    )

    return response.json()


def update_item_iteration(item_id, field_id, iteration_id):
    """Update a project item's iteration"""
    mutation = """
    mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $iterationId: String!) {
      updateProjectV2ItemFieldValue(
        input: {
          projectId: $projectId
          itemId: $itemId
          fieldId: $fieldId
          value: { 
            iterationId: $iterationId
          }
        }
      ) {
        projectV2Item {
          id
        }
      }
    }
    """

    variables = {
        "projectId": project_id,
        "itemId": item_id,
        "fieldId": field_id,
        "iterationId": iteration_id,
    }

    response = requests.post(
        GITHUB_API,
        json={"query": mutation, "variables": variables},
        headers={"Authorization": f"Bearer {GITHUB_TOKEN}"},
    )

    return response.json()


def update_item_priority(item_id, field_id, option_id):
    """Update a project item's priority"""
    mutation = """
    mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $optionId: String!) {
      updateProjectV2ItemFieldValue(
        input: {
          projectId: $projectId
          itemId: $itemId
          fieldId: $fieldId
          value: { 
            singleSelectOptionId: $optionId
          }
        }
      ) {
        projectV2Item {
          id
        }
      }
    }
    """

    variables = {
        "projectId": project_id,
        "itemId": item_id,
        "fieldId": field_id,
        "optionId": option_id,
    }

    response = requests.post(
        GITHUB_API,
        json={"query": mutation, "variables": variables},
        headers={"Authorization": f"Bearer {GITHUB_TOKEN}"},
    )

    return response.json()


# Load the prioritized issues
with open("prioritized_issues.json", "r") as f:
    prioritized_issues = json.load(f)

# Get project fields and field IDs
fields_data = get_project_fields()
iteration_field = None
priority_field = None
iterations = {}
priorities = {}

# Extract iteration and priority fields
for field in fields_data["data"]["user"]["projectV2"]["fields"]["nodes"]:
    if field and field.get("name") == "Iteration":
        iteration_field = field
        for iteration in field["configuration"]["iterations"]:
            iterations[iteration["title"]] = iteration["id"]
    elif field and field.get("name") == "Priority":
        priority_field = field
        for option in field["options"]:
            priorities[option["name"]] = option["id"]

if not iteration_field or not priority_field:
    print("Error: Couldn't find required fields in project")
    exit(1)

# Get project ID and items
project_query = """
query($owner: String!, $number: Int!) {
  user(login: $owner) {
    projectV2(number: $number) {
      id
    }
  }
}
"""

project_response = requests.post(
    GITHUB_API,
    json={
        "query": project_query,
        "variables": {"owner": OWNER, "number": PROJECT_NUMBER},
    },
    headers={"Authorization": f"Bearer {GITHUB_TOKEN}"},
)

project_data = project_response.json()
project_id = project_data["data"]["user"]["projectV2"]["id"]

# Update each item
print("Starting updates...")
for issue in prioritized_issues:
    iteration = issue["iteration"]
    priority = issue["priority"]

    print(f"Processing '{issue['title']}'...")

    # Update iteration
    if iteration in iterations:
        print(f"  Updating iteration to '{iteration}'")
        result = update_item_iteration(
            issue["id"], iteration_field["id"], iterations[iteration]
        )
        if "errors" in result:
            print(f"  Error updating iteration: {result['errors']}")
        sleep(1)  # Rate limiting delay

    # Update priority
    if priority in priorities:
        print(f"  Updating priority to '{priority}'")
        result = update_item_priority(
            issue["id"], priority_field["id"], priorities[priority]
        )
        if "errors" in result:
            print(f"  Error updating priority: {result['errors']}")
        sleep(1)  # Rate limiting delay

print("Updates complete!")
