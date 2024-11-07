#!/bin/bash

# Check if a file argument was provided
if [ $# -eq 0 ]; then
    echo "Error: Please provide a JSON file as an argument"
    echo "Usage: ./create_issues.sh <input_file.json>"
    exit 1
fi

input_file="$1"

# Check if the file exists
if [ ! -f "$input_file" ]; then
    echo "Error: File '$input_file' not found"
    exit 1
fi

# Function to create an issue
create_issue() {
    local title="$1"
    local body="$2"
    
    echo "Creating issue: $title"
    
    # Create the issue
    issue_number=$(gh issue create --title "$title" --body "$body" --repo "yogimathius/capstone")
    
    if [ $? -eq 0 ]; then
        echo "Issue #$issue_number created successfully"
        
        # Add to project
        echo "Adding issue to project #11"
        gh project item-add 11 --owner yogimathius --url "$issue_number"
        
        if [ $? -eq 0 ]; then
            echo "Successfully added issue #$issue_number to project"
        else
            echo "Failed to add issue to project"
        fi
    else
        echo "Failed to create issue"
    fi
}

# Read JSON and create issues
jq -c '.issues[]' "$input_file" | while read -r issue; do
    title=$(echo "$issue" | jq -r '.title')
    description=$(echo "$issue" | jq -r '.description')
    
    # Check if tasks exist
    tasks=$(echo "$issue" | jq -r '.tasks[]?' 2>/dev/null)
    
    if [ ! -z "$tasks" ]; then
        # Format tasks as a checklist
        body="$description\n\n### Tasks:\n$(echo "$tasks" | sed 's/^/- [ ] /')"
    else
        body="$description"
    fi
    
    create_issue "$title" "$body"
    
    # Add a small delay to avoid rate limiting
    sleep 1
done