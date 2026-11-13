# AgentHub

A universal manager for discovering, launching, and configuring locally installed AI coding agents.

AgentHub is a Rust application that scans the current machine for agent-like command-line tools and provides a single entry point to launch them in project workspaces.

**Discovery is dynamic.** AgentHub does not ship a fixed catalogue of product names. It inspects executable locations and uses generic signals to identify likely coding-agent tools. Unknown executables can remain visible as discovered candidates.

## Current status

Early development. The first milestone focuses on a cross-platform discovery core and generic process launcher.

### Implemented

- Scan executable files available through PATH.
- Add explicitly configured executable paths.
- Deduplicate executable paths.
- Apply generic name-based recognition signals.
- Launch a discovered executable in a selected workspace.
- Human-readable and JSON output.

### Planned

- Application and configuration-directory discovery.
- Adapter/plugin API for agent-specific capabilities.
- Provider and model management.
- Secure credential storage.
- Scheduled model/configuration synchronization.
- Interactive TUI.

## Build

Requires a recent stable Rust toolchain.

    cargo build --release

## Usage

    agenthub scan
    agenthub scan --json
    agenthub list
    agenthub run <name-or-id> --workspace .

Arguments after the separator are passed to the selected executable.

## Configuration

The optional configuration file is stored in the operating system's user config directory, under agenthub/config.json.

Example:

    {
      "custom_executables": ["/opt/tools/my-private-agent"]
    }

## Limitations

- A machine-wide scan cannot prove that every installed AI agent has been found.
- The initial scanner inspects PATH; application bundles and agent-specific configuration directories are not scanned yet.
- Generic recognition is heuristic and can produce false positives or miss tools with unusual names.
- The generic process adapter can launch executables, but it does not expose agent-specific settings or provider/model controls.

## License

MIT. See LICENSE.
