#!/bin/bash

# Configuration
GITHUB_TOKEN="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"
OWNER="yogimathius"
REPO="praxis-forge"
PROJECT_NUMBER=11

# GraphQL query
query='
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
              ... on ProjectV2ItemFieldDateValue {
                date
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
          content {
            ... on Issue {
              title
              number
              state
              body
              url
            }
          }
        }
      }
    }
  }
}'

# Get first 100 items
first_response=$(curl -s -X POST \
  -H "Authorization: bearer $GITHUB_TOKEN" \
  -H "Content-Type: application/json" \
  https://api.github.com/graphql \
  -d @- <<EOF
{
  "query": $(echo "$query" | jq -R -s .),
  "variables": {
    "owner": "$OWNER",
    "number": $PROJECT_NUMBER,
    "cursor": null
  }
}
EOF
)

# Debug first response
echo "First response:"
echo "$first_response" | jq '.'

# Get the cursor for the next page
cursor=$(echo "$first_response" | jq -r '.data.user.projectV2.items.pageInfo.endCursor')
echo "Cursor: $cursor"

# Get next 100 items
second_response=$(curl -s -X POST \
  -H "Authorization: bearer $GITHUB_TOKEN" \
  -H "Content-Type: application/json" \
  https://api.github.com/graphql \
  -d @- <<EOF
{
  "query": $(echo "$query" | jq -R -s .),
  "variables": {
    "owner": "$OWNER",
    "number": $PROJECT_NUMBER,
    "cursor": "$cursor"
  }
}
EOF
)

# Debug second response
echo "Second response:"
echo "$second_response" | jq '.'

# Combine both responses with error checking
if echo "$first_response" | jq -e '.data.user.projectV2.items.nodes' > /dev/null; then
    combined_items=$(echo "$first_response" | jq -r '.data.user.projectV2.items.nodes')
    if echo "$second_response" | jq -e '.data.user.projectV2.items.nodes' > /dev/null; then
        second_items=$(echo "$second_response" | jq -r '.data.user.projectV2.items.nodes')
        all_items=$(echo "[$combined_items, $second_items]" | jq 'add')
    else
        all_items=$combined_items
    fi

    # Save response to JSON file (with error handling)
    if [ -n "$all_items" ]; then
        echo "$all_items" | jq 'map({
            id: .id,
            title: (.fieldValues.nodes[] | select(.field.name == "Title") | .text),
            status: ((.fieldValues.nodes[] | select(.field.name == "Status") | .name) // "No Status"),
            component: ((.fieldValues.nodes[] | select(.field.name == "Component") | .name) // "Uncategorized"),
            iteration: ((.fieldValues.nodes[] | select(.field.name == "Iteration") | .title) // "No Iteration")
        }) | map(select(.status != "Done" and .status != "Frozen")) | sort_by(.iteration)' > issues.json
        
        # Count and display the number of saved issues
        issue_count=$(cat issues.json | jq '. | length')
        echo "Saved $issue_count issues to issues.json"
    else
        echo "Error: No valid data found in the response"
        echo "Please check if:"
        echo "1. Your GitHub token has the right permissions (needs read:project access)"
        echo "2. The project number ($PROJECT_NUMBER) is correct"
        echo "3. The owner ($OWNER) is correct"
    fi
else
    echo "Error: No valid data found in the responses"
    echo "First response error:"
    echo "$first_response" | jq '.errors'
    echo "Second response error:"
    echo "$second_response" | jq '.errors'
fi
