#!/bin/bash

# be stricter
set -eou pipefail

# Upgrade a list of outdated librustzcash crates. Build the project. Return an env var indicating if the build was successful.
#
# Takes the following function args:
# $1 - outdated uniffi librustzcash dependencies where the version is not latest, in format - "crate_name;...".
# $2 - uniffi-zcash-lib Cargo.toml path
upgrade() {
	local outdated_libs=$1
	local uniffi_cargo_path=$2
	if [[ -z "$outdated_libs" || -z "$uniffi_cargo_path" ]]; then
		echo "required parameter for upgrade_and_build() is empty" 1>&2
		exit 1
	fi

	IFS=';' read -ra arr <<<"$outdated_libs"
	cmd_args=("cargo" "upgrade")
	for lib_name in "${arr[@]}"; do
		if [[ -z "$lib_name" ]]; then
			continue
		fi
		cmd_args+=("-p" "$lib_name")
	done
	cmd_args+=("-i" "--manifest-path" "./uniffi-zcash-lib/lib/Cargo.toml")
	"${cmd_args[@]}"
}
