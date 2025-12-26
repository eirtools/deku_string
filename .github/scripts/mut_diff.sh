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

mutdiff origin/main...HEAD --output=git.diff -- ':(exclude)tests/data/*' > /dev/null
