# GitHub CLI Terraform Module Extension

WARNING: This project is still very much a work in progress. Use at your own risk.

Finds, installs and removes Terraform modules within a GitHub organization, assuming they follow a predictable naming convention:

```none
<organization>/terraform-<provider>-<name>-module
```

## Installation

```bash
gh extension install https://github.com/yhakbar/gh-tf-mod
```

## Usage

```bash
$ gh tf-mod -h
Usage:
    gh tf-mod [-hvopc] <command>

    flags:
        -h, --help          Prints help information
        -v, --version       Prints version information

    global flags:
        -o, --organization  The GitHub organization to use
        -p, --provider      The provider to use
        -c, --config        The path to the config file

    commands:
        ls
        add
        rm
        version
        help
```
