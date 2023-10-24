#!/bin/bash

# be stricter
set -eou pipefail

# Run the public API diff tool from uniffi-zcash-cli
#
# Takes the following function args:
# $1 - outdated librustzcash dependency where the version is not latest, in format - "crate_name;..."
# $2 - uniffi-zcash-lib Cargo.toml path
# $3 - absolute path of the directory, that's passed to the diff tool
#
# Exports:
# ${lib_name}_colored.diff file for every outdated library - for output in the github workflow console
# ${lib_name}.diff file for every outdated library - for output in the issue

diff() {
	local outdated_libs="$1"
	local uniffi_cargo_path="$2"
	local grep_dir="$3"
	local librustzcash_abs_path="$4"
	if [[ -z "$outdated_libs" || -z "$uniffi_cargo_path" || -z "$grep_dir" ]]; then
		echo "required parameter for diff() is empty" 1>&2
		exit 1
	fi

	IFS=';' read -ra arr <<<"$outdated_libs"
	for lib_name in "${arr[@]}"; do
		if [[ -z "$lib_name" ]]; then
			continue
		fi

		# this is faster than "cargo outdated", especially in a loop
		local lib_latest_version
		lib_latest_version=$(curl --silent "https://crates.io/api/v1/crates/$lib_name" | jq -r '.crate.max_stable_version')
		# this is faster than "cargo outdated", especially in a loop
		local lib_current_version
		lib_current_version=$(cargo metadata --format-version=1 -q --manifest-path="$uniffi_cargo_path" |
			jq -r --arg lib_name "$lib_name" '.packages[] | select(.name == $lib_name) | .version')

		# write the diffs to files, which we show in a separate step for better readability
		# for colored output ANSI color codes are written in the file and can't be rendered in markdown

		cargo run \
			--manifest-path="$uniffi_cargo_path" \
			-p uniffi-zcash-cli diff \
			--grep-dir "$grep_dir" \
			--lib-name "$lib_name" \
			--lib-old-version "$lib_current_version" \
			--lib-new-version "$lib_latest_version" \
			--color always \
			--librustzcash-path "$librustzcash_abs_path" >"${lib_name}_colored.diff"

		# non-colored output
		cargo run \
			--manifest-path="$uniffi_cargo_path" \
			-p uniffi-zcash-cli diff \
			--grep-dir "$grep_dir" \
			--lib-name "$lib_name" \
			--lib-old-version "$lib_current_version" \
			--lib-new-version "$lib_latest_version" \
			--color never \
			--librustzcash-path "$librustzcash_abs_path" >"${lib_name}.diff"
	done
}
