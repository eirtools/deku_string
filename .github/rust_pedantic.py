#!/usr/bin/env python
import itertools
import subprocess
import sys

allowed_in_project = (
    "clippy::integer_division_remainder_used",  # maybe important in some contexts.
    "clippy::integer_division",  # maybe important in some contexts.
    "clippy::arithmetic_side_effects",  #  maybe important in some contexts.
    "clippy::multiple_inherent_impl",  # In current code it's organized well.
)

allowed_globally = (
    "clippy::blanket_clippy_restriction_lints",  # Enabling pedantic clippy.
    "clippy::allow_attributes",  # Just don't allow them without reasons.
    "clippy::implicit_return",  # Syntax sugar.
    "clippy::question_mark_used",  # Syntax sugar.
    "clippy::pub_with_shorthand",  # Syntax sugar.
    "clippy::default_numeric_fallback",  # Syntax sugar.
    "clippy::mod_module_files",  # Code layout design.
    "clippy::pub_use",  # Code layout design.
    "clippy::redundant_pub_crate",  # Code layout design.
    "clippy::single_char_lifetime_names",  # It's too common to use one char names
    "clippy::missing_trait_methods",  # There's little reason to re_implement trait methods
    "clippy::single_call_fn",  # Why this could be a problem? readability counts
    "clippy::arbitrary_source_item_ordering",  # many mixed lints
    # "clippy::min_ident_chars",  # It's too common to use one_char names…
)

allowed = allowed_globally + allowed_in_project


# Enable all warnings (except deprecated)
warning = (
    "clippy::cargo",
    "clippy::complexity",
    "clippy::correctness",
    "clippy::nursery",
    "clippy::pedantic",
    "clippy::perf",
    "clippy::restriction",
    "clippy::style",
    "clippy::suspicious",
)

# Deny
deny = ()

# Forbidden
forbidden = ()


def interlace(key, iterable):
    key_iter = itertools.repeat(key)
    return itertools.chain.from_iterable(zip(key_iter, iterable))


def clippy_rules():
    key_allowed = interlace("-A", allowed)
    key_warning = interlace("-W", warning)
    key_deny = interlace("-D", deny)
    key_forbidden = interlace("-F", forbidden)
    return itertools.chain.from_iterable(
        (key_warning, key_deny, key_forbidden, key_allowed)
    )


def call_clippy():
    color = ["--color", "always"]
    targets = ["--all-targets", "--all-features"]
    future = ["--future-incompat-report"]
    clippy_cli = (
        ["cargo", "clippy"] + color + targets + future + ["--"] + list(clippy_rules())
    )
    print("Calling clippy:", " ".join(clippy_cli))
    result = subprocess.run(clippy_cli)
    if result.returncode != 0:
        sys.exit(result.returncode)


def call_rustfmt():
    config_path = "rustfmt_pedantic.toml"
    pedantic_cli = ["cargo", "+nightly", "fmt", "--", "--config-path", config_path]
    print("Calling nightly rustfmt:", " ".join(pedantic_cli))
    result = subprocess.run(pedantic_cli)
    if result.returncode != 0:
        sys.exit(result.returncode)


def main():
    call_clippy()
    call_rustfmt()


if __name__ == "__main__":
    main()
