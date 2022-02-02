# GitHub CLI Terraform Module Extension

WARNING: This project is still very much a work in progress. Use at your own risk.

Finds, installs and removes Terraform modules within a GitHub organization, assuming they follow a predictable naming convention:

```none
<organization>/terraform-<provider>-<name>-module
```

These modules should have semver releases, with a compressed, minified copy of the module named `release.tar.gz` as an asset.

## Installation

```bash
gh extension install https://github.com/yhakbar/gh-tf-mod
```

## Upgrades

```bash
gh extension upgrade tf-mod
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
        config
        version
        help
```

## Config

There is a config file that is located at `~/.config/gh-tf-mod/config.json` by default. You can run the following to populate it:

```bash
$ gh tf-mod config
No config file found at /Users/yhakbar/.config/gh-tf-mod/config.json. Creating...
Which organization would you like to use with gh-tf-mod? Leave blank to use your personal GitHub user: 
Defaulting to logged in user: yhakbar.
Would you like to set a default provider? Leave blank to avoid setting a default provider: aws
```

You don't have to set this config file if you don't want to, but it can save you some typing.

By default, the organization used will be your personal GitHub user.

If you don't specify a default provider, make sure to either pass in the provider, or adjust your usage to include it.

e.g.

```bash
$ gh tf-mod ls aws-iam-role
...
# or
$ gh tf-mod ls -p aws iam-role
...
```

## List

## List Terraform Modules

```bash
$ gh tf-mod ls
terraform-aws-lambda-module
terraform-aws-iam-role-module
```

## List Latest Release of Terraform Module

```bash
$ gh tf-mod ls aws-iam-role
0.0.3
```

## List All Releases of Terraform Module

```bash
$ gh tf-mod ls aws-iam-role -v
0.0.1
0.0.2
0.0.3
```
