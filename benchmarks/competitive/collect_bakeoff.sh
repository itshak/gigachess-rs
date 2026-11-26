#!/usr/bin/env bash
# Collect immutable run artifacts, never overwrite prior evidence.
set -euo pipefail
repo=itshak/gigachess-rs
out=${1:?output directory required}
shift
for id in "$@"; do
  dest="$out/$id"
  mkdir -p "$dest"
  gh api "repos/$repo/actions/runs/$id/artifacts?per_page=100" --jq '.artifacts[]|select(.expired==false)|.name' |
    while IFS= read -r name; do
      if [[ ! -d "$dest/$name" ]]; then
        gh run download "$id" --repo "$repo" --dir "$dest/$name" --name "$name"
      fi
    done
  gh run view "$id" --repo "$repo" --json status,conclusion,headSha,jobs,url > "$dest/run.json"
done
