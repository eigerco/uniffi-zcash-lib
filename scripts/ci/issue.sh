#!/bin/bash

# be stricter
set -eou pipefail

# Use jq to get the outdated libs from the "cargo outdated" JSON string and generate Github issue labels
#
# Takes the following function args:
# $1 - "cargo outdated" JSON string
#
# Returns:
# - The labels that are used when searching for or creating a Github issue, in format 'lib_name-current_ver-latest_ver'
generate_issue_labels() {
	local outdated_libs_json="$1"
	if [[ -z "$outdated_libs_json" ]]; then
		echo "required parameter for generate_issue_labels() is empty" 1>&2
		exit 1
	fi

	# Export ISSUE_LABELS in format - "crate_name-current_version-latest_version;..."
	local issue_labels
	issue_labels=$(echo "$outdated_libs_json" |
		jq -r 'select(.crate_name | startswith("uniffi-") or .=="zcash").dependencies[] | select(.project != .latest) | (.name+"-"+.project+"-"+.latest)' |
		sort -u |
		tr '\n' ';')

	echo "$issue_labels"
}

# Get the issue URL from the "gh issue" command JSON response
#
# Takes the following function args:
# $1 - issue response from "gh issue" command in JSON format
#
# Returns:
# - The issue URL
issue_url_from_json() {
	local issue_json="$1"

	if [[ -z "$issue_json" ]]; then
		echo "required parameter for issue_url_from_json() is empty" 1>&2
		exit 1
	fi

	local issue_url
	issue_url=$(echo "$issue_json" | jq -r '.[] | .url')

	echo "$issue_url"
}

# Search for an issue from it's labels
#
# Takes the following function args:
# $1 - issue labels in format 'lib_name-current_ver-latest_ver'
#
# Returns:
# - The response of the "gh issue" command in JSON format
get_issue_by_labels() {
	local issue_labels="$1"

	if [[ -z "$issue_labels" ]]; then
		echo "required parameter for get_issue_by_labels() is empty" 1>&2
		exit 1
	fi

	IFS=';' read -ra arr <<<"$issue_labels"
	local cmd_args
	cmd_args=("gh" "issue" "list" "--repo" "$GITHUB_REPOSITORY" "--json" "body,url")
	for label in "${arr[@]}"; do
		if [[ -z "$label" ]]; then
			continue
		fi
		cmd_args+=("--label" "$label")
	done

	local issues_json
	issues_json=$("${cmd_args[@]}")

	echo "$issues_json"
}

# Creates an issue with certain labels
#
# Takes the following function args:
# $1 - issue labels in format 'lib_name-current_ver-latest_ver'
create_issue_with_labels() {
	local issue_labels="$1"

	# first we need to create the labels, before using them
	IFS=';' read -ra arr <<<"$issue_labels"
	for label in "${arr[@]}"; do
		gh label create "$label" --repo "$GITHUB_REPOSITORY" --force
	done

	# create the issue with certain labels
	local cmd_args
	cmd_args=("gh" "issue" "create" "--repo" "$GITHUB_REPOSITORY" "--title" "New versions of librustzcash. Please review." "--body-file" "issue_body")
	IFS=';' read -ra arr <<<"$issue_labels"
	for label in "${arr[@]}"; do
		cmd_args+=("--label" "$label")
	done
	"${cmd_args[@]}"
}
