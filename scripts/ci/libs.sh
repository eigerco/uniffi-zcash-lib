#!/bin/bash

# be stricter
set -eou pipefail

# Get librustzcash libraries used in uniffi-zcash-lib
#
# Takes the following function args:
# $1 - librustzcash Cargo.toml path
# $2 - uniffi-zcash-lib Cargo.toml path
#
# Returns:
# - The librustzcash libraries that are used as dependencies in uniffi-zcash-lib
get_libs() {
	local librustzcash_cargo_path="$1"
	local uniffi_cargo_path="$2"
	if [[ -z "$librustzcash_cargo_path" || -z "$uniffi_cargo_path" ]]; then
		echo "required parameter for get_libs() is empty" 1>&2
		exit 1
	fi

	local librustzcash_packages
	librustzcash_packages=$(cargo metadata --format-version=1 --no-deps --quiet --manifest-path="$librustzcash_cargo_path" |
		jq -r '.packages[] | .name' | tr '\n' '|' | sed 's/|$//')

	local output
	output=$(cargo metadata --format-version=1 --no-deps --manifest-path="$uniffi_cargo_path" |
		jq -r '.packages[] | .dependencies[] | .name' |
		grep -Ei "$librustzcash_packages" |
		sort -u |
		tr '\n' ';')

	echo "$output"
}

# Use jq to get the outdated libs from the "cargo outdated" JSON string
#
# Takes the following function args:
# $1 - "cargo outdated" JSON string
#
# Returns:
# - outdated uniffi librustzcash dependency where the version is not latest, in format - "crate_name;..."
get_outdated_libs() {
	local outdated_libs_json="$1"
	if [[ -z "$outdated_libs_json" ]]; then
		echo "required parameter for get_outdated_libs() is empty" 1>&2
		exit 1
	fi

	echo "$outdated_libs_json" |
		jq -r 'select(.crate_name | startswith("uniffi-") or .=="zcash").dependencies[] | select(.project != .latest) | .name' |
		sort -u |
		tr '\n' ';'
}

# Get the outdated librustzcash dependecies, used in uniffi-zcash-lib
#
# Takes the following function args:
# $1 - used librustzcash packages, separated by ';'
# $2 - uniffi-zcash package Cargo.toml
#
# Returns:
# - cargo outdated command response in JSON format
get_outdated_libs_json() {
	local used_libs="$1"
	local cargo_path="$2"
	if [[ -z "$used_libs" || -z "$cargo_path" ]]; then
		echo "required parameter for get_outdated_libs_json() is empty" 1>&2
		exit 1
	fi

	IFS=';' read -ra arr <<<"$used_libs"
	local cmd_args
	cmd_args=("cargo" "outdated" "--manifest-path" "$cargo_path" "--format" "json")
	for lib_name in "${arr[@]}"; do
		if [[ -z "$lib_name" ]]; then
			continue
		fi
		cmd_args+=("-p" "$lib_name")
	done

	local outdated_libs_json
	outdated_libs_json=$("${cmd_args[@]}")

	echo "$outdated_libs_json"
}
