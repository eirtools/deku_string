#!/bin/sh
# the output is `git diff` without moved and unchanged files.
# For moved and changed files only diff is shown.
# taken from https://stackoverflow.com/a/44394214

mutdiff() {
    base="$1"

    git diff "$@" -- . \
        $( (git diff "$base" -M100% --diff-filter=R --name-only \
            && git diff "$base" -M100% --diff-filter=R --name-only -R) \
          | sed 's|^|:(exclude)|' )
}

base_ref="${1:-main}"
output_filename="${2:-git.diff}"

mutdiff "origin/${base_ref}...HEAD" "--output=$output_filename" -- ':(exclude)tests/data/*' > /dev/null
