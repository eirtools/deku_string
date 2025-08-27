#!/usr/bin/env python3
from collections.abc import Iterator
import argparse
import os
import re
import subprocess
import tomllib
from typing import Any

VERSION_RE = re.compile(r"\d+\.\d+\.\d+")
CRATE_NAME_RE = re.compile(r"[a-zA-Z0-9_-]+")


def parse_args():
    parser = argparse.ArgumentParser(
        description="Process file with to tools to install."
    )
    parser.add_argument(
        "-f",
        "--tools-file",
        help="Path to the TOML file containing tool versions",
        dest="toml_file",
    )
    parser.add_argument("-s", "--section", help="Section to use")
    parser.add_argument(
        "--crates-toml",
        help="crates.toml location (can be empty)",
        default=os.path.expanduser("~/.cargo/.crates.toml"),
    )
    parser.add_argument("--default-datasource", help="Default data source")
    parser.add_argument("--force", help="Force reinstall", action="store_true")
    parser.add_argument("-d", "--debug", help="Debug information", action="store_true")
    parser.add_argument("-n", "--dry-run", help="Dry run", action="store_true")
    parser.add_argument("-q", "--quiet", help="Minimize output", action="store_true")
    return parser.parse_args()


def sanitize_version(version: Any) -> str | None:
    if not isinstance(version, str):
        return None

    version = version.strip()
    if VERSION_RE.fullmatch(version):
        return version
    return None


def sanitize_tool_name(tool_name: str) -> str | None:
    tool_name = tool_name.strip()
    if CRATE_NAME_RE.fullmatch(tool_name):
        return tool_name
    return None


def sanitize_datasource(datasource: Any) -> str | None:
    if not isinstance(datasource, str):
        return None

    return datasource.strip()


def read_tools_file(
    toml_file, section: str, default_datasource: str | None
) -> Iterator[tuple[str, str, str | None]]:
    with open(toml_file) as f:
        data = tomllib.loads(f.read())

    tools = data.get(section, {})

    for tool, value in tools.items():
        if isinstance(value, str):
            yield (tool, value, default_datasource)
            continue
        elif isinstance(value, dict):
            version = value.get("version", None)
            datasource = value.get("", default_datasource)

            if isinstance(version, str) and isinstance(datasource, (str, type(None))):
                yield (tool, version, datasource)


def read_tools(
    toml_file, section: str, default_datasource: str | None
) -> Iterator[tuple[str, str, str | None]]:
    return filter(
        lambda x: bool(x[0]) and bool(x[1]),
        map(
            lambda x: (
                sanitize_tool_name(x[0]),
                sanitize_version(x[1]),
                sanitize_datasource(x[2]),
            ),
            read_tools_file(toml_file, section, default_datasource),
        ),
    )


def install_tool_crate(
    crate_name: str,
    version: str,
    force: bool,
    quiet: bool,
    debug: bool,
    dry_run: bool,
    installed_crates: dict[Any, Any],
) -> None:
    if quiet:

        def verbose_print(*args, **kwargs): ...
    else:
        verbose_print = print

    if crate_name in installed_crates:
        installed_version = installed_crates[crate_name]

        if installed_version == version:
            verbose_print(f"✅ {crate_name} already installed at {version}")
            return
        else:
            verbose_print(
                f"🔁 {crate_name} version mismatch (found: {installed_version}, expected: {version}), reinstalling..."
            )
    else:
        verbose_print(f"> {crate_name} not found, installing...")

    command = ["cargo", "binstall", "--no-confirm", f"{crate_name}@{version}"]

    if force:
        command.append("--force")

    verbose_print(f"🔧 Installing {crate_name}@{version}")

    if debug:
        print(f"⚙️ Running install command: {' '.join(command)}")

    if not dry_run:
        result = subprocess.run(command, check=True, text=True)
        result.check_returncode()

    verbose_print(f"Successfully installed {crate_name} version {version}")


data_sources = {
    "crate": install_tool_crate,
}


def install_tool(
    tool_name: str,
    version: str,
    datasource: str,
    force: bool,
    quiet: bool,
    debug: bool,
    dry_run: bool,
    installed_crates: dict[Any, Any],
) -> None:
    install_function = data_sources.get(datasource)
    if not install_function:
        if not quiet:
            print(
                f"WARNING: Unable to find handler for datasource: {datasource} for {tool_name}"
            )
        return

    install_function(tool_name, version, force, quiet, debug, dry_run, installed_crates)


def parse_installed_cargo(crates_toml: str) -> Iterator[tuple[str, str]]:
    if not crates_toml:
        return

    with open(crates_toml) as f:
        data = tomllib.loads(f.read())

    data = data.get("v1")

    if not data:
        return

    for key in data.keys():
        (crate, version, _) = key.split(" ", 2)
        yield (crate, version)


def main() -> None:
    args = parse_args()

    installed_crates = dict(parse_installed_cargo(args.crates_toml))
    if not args.quiet:
        print(
            f"📦 Installing cargo tools from {args.toml_file}/{args.section} using cargo-binstall"
        )

    if args.debug:
        print("📦 Installed cargo packages:")
        for crate, version in installed_crates.items():
            print(f"{crate} {version}")

    for tool_name, version, datasource in read_tools(
        args.toml_file, args.section, args.default_datasource
    ):
        install_tool(
            tool_name=tool_name,
            version=version,
            datasource=datasource,
            force=args.force,
            quiet=args.quiet,
            debug=args.debug,
            dry_run=args.dry_run,
            installed_crates=installed_crates,
        )


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError:
        os.exit(1)
