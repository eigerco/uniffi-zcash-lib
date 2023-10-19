#!/bin/bash

# be stricter
set -eou pipefail

# Print the public API diff from the cli tool in the Github workflow summary
#
# Takes the following function args:
# $1 - outdated librustzcash dependency where the version is not latest, in format - "crate_name;..."
# $2 - the URL, pointing to the Github workflow console output of the public API diff cli tool
print_workflow_diff() {
	local outdated_libs="$1"
	local diff_result_workflow_url="$2"

	echo "# :warning: New versions of librustzcash libraries are present :warning:"
	echo "You can view a better colored result of the diff in the **[CI logs]($diff_result_workflow_url)**."

	IFS=';' read -ra arr <<<"$outdated_libs"
	for lib_name in "${arr[@]}"; do
		if [[ -z "$lib_name" ]]; then
			continue
		fi

		local lib_latest_version
		lib_latest_version=$(curl --silent "https://crates.io/api/v1/crates/$lib_name" |
			jq -r '.crate.max_stable_version')

		local lib_current_version
		lib_current_version=$(cargo metadata --format-version=1 -q --manifest-path=./uniffi-zcash-lib/lib/Cargo.toml |
			jq -r ".packages[] | select(.name == \"$lib_name\") | .version")

		echo "## ${lib_name}"
		echo "\`CURRENTLY USED VERSION\`    :arrow_right: ${lib_current_version}"
		echo "\`LATEST PUBLISHED VERSION\`  :arrow_right: ${lib_latest_version}"
		echo ""
		echo "\`\`\`diff"
		cat "$lib_name".diff
		echo ""
		echo "\`\`\`"
	done
}

# Print the public API diff from the cli tool in the Github workflow summary
#
# Takes the following function args:
# $1 - a boolean string "true" or "false", indicating if the build failed
# $2 - the URL, pointing to the Github workflow console output of the uniffi-zcash-lib build
print_workflow_build_result() {
	local build_failing="$1"
	local build_result_workflow_url="$2"

	if [[ "$build_failing" == "true" ]]; then
		echo "# :warning: Build fails after bumping to the newer versions with the following output: :warning: "
		echo "You can also view the build result in the **[CI logs]($build_result_workflow_url)**."
		echo "\`\`\`"
		cat build_output
		echo "\`\`\`"
	fi

	if [[ "$build_failing" == "false" ]]; then
		echo "# :white_check_mark: Build doesn't fail when bumping to the newer versions :white_check_mark: "
	fi

}

# Print the public API diff from the cli tool in the Github workflow summary
#
# Takes the following function args:
# $1 - outdated librustzcash dependency where the version is not latest, in format - "crate_name;..."
# $2 - the URL, pointing to the Github workflow console output of the public API diff cli tool
print_issue_diff() {
	local outdated_libs="$1"
	local diff_result_workflow_url="$2"

	echo "# :warning: New versions of librustzcash libraries are present :warning: "
	echo "You can view the also public API diff between versions in the **[CI logs]($diff_result_workflow_url)**."

	IFS=';' read -ra arr <<<"$outdated_libs"
	for lib_name in "${arr[@]}"; do
		if [[ -z "$lib_name" ]]; then
			continue
		fi

		local lib_latest_version
		lib_latest_version=$(curl --silent "https://crates.io/api/v1/crates/$lib_name" |
			jq -r '.crate.max_stable_version')

		local lib_current_version
		lib_current_version=$(cargo metadata --format-version=1 -q --manifest-path=./uniffi-zcash-lib/lib/Cargo.toml |
			jq -r ".packages[] | select(.name == \"$lib_name\") | .version")

		echo "## ${lib_name}"
		echo "\`CURRENTLY USED VERSION\`    :arrow_right: ${lib_current_version}"
		echo "\`LATEST PUBLISHED VERSION\`  :arrow_right: ${lib_latest_version}"
		echo ""
		echo "\`\`\`diff"
		cat "$lib_name".diff
		echo "\`\`\`"
	done
}

# Print the public API diff from the cli tool in the Github workflow summary
#
# Takes the following function args:
# $1 - a boolean string "true" or "false", indicating if the build failed
# $2 - the URL, pointing to the Github workflow console output of the uniffi-zcash-lib build
print_issue_build_result() {
	local build_failing="$1"
	local build_result_workflow_url="$2"

	if [[ "$build_failing" == "false" ]]; then
		echo "# :white_check_mark: Build doesn't fail after updating to the newer versions :white_check_mark: "
	fi

	if [[ "$build_failing" == "true" ]]; then
		echo "# :warning: Build fails after bumping to the newer versions with the following output: :warning: "
		echo "You can view the also public API diff between versions in the **[CI logs]($build_result_workflow_url)**."
		echo "\`\`\`"
		grep -v "Compiling" <build_output
		echo "\`\`\`"
	fi
}

# If the issue_body file has more than 65300 characters (close to the Github issue body size limit), add "..." to the issue text
# and append a message that indicated that some constraint has been reached. Point to the workflow summary, which as of now, doesn't have such limits.
cut_issue_body() {
	# if the body has reached github issue body limit, then close the ``` and show message that limit is reached
	if [[ $(wc -m <issue_body) -gt 65300 ]]; then
		head -c 65300 <issue_body >temp_issue_body && mv temp_issue_body issue_body
		head -n -1 issue_body # removes the last row from the file (expected to be ```), so we can continue with the "...".
		echo "..." >>issue_body
		echo "" >>issue_body
		echo "\`\`\`" >>issue_body
		echo "## :construction: The Github issue body size limit was reached. Please visit the summary link at the top of the issue for the full message :construction: " >>issue_body
	fi
}

# Echoes the build job URL, which points to the build logs in the Github CI console
get_build_job_url() {
	local result
	result=$(gh run \
		--repo "$GITHUB_REPOSITORY" view "$GITHUB_RUN_ID" \
		--json jobs \
		--jq '.jobs[] | select(.name == "'$GITHUB_JOB'") | .url, (.steps[] | select(.name == "Show public API diffs") | "#step:\(.number):1")' |
		tr -d "\n")

	echo "$result"
}

# Echoes the diff job URL, which points to the colored public API diff logs in the Github CI console
get_diff_job_url() {
	local result
	result=$(gh run \
		--repo "$GITHUB_REPOSITORY" view "$GITHUB_RUN_ID" \
		--json jobs \
		--jq '.jobs[] | select(.name == "'$GITHUB_JOB'") | .url, (.steps[] | select(.name == "Check if uniffi-zcash-lib build is failing") | "#step:\(.number):1")' |
		tr -d "\n")

	echo "$result"
}
