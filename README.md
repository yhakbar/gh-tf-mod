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

Please use the `-h` flag for usage info, including on subcommands.

If the information is not clear, please consider opening an issue or a pull request on this repository.

## Config

You can optionally use a config file by placing a config file at `.config/gh-tf-mod.yaml`. Running the following populates it based on the `-o|--organization` and `-p|--provider` flags:

```bash
gh tf-mod config -o github-organization -p terraform-provider
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

Use the `ls` subcommand without any target to list all modules in a GitHub organization.

```bash
$ gh tf-mod ls
+----------+----------+
| Name     | Provider |
+==========+==========+
| lambda   | aws      |
+----------+----------+
| iam-role | aws      |
+----------+----------+
+-------+--------------+
| Repos | Hidden Repos |
+=======+==============+
| 3     | 1            |
+-------+--------------+
```

Use the `-l|--long` flag to see all the optional data that is available:

```bash
$ gh tf-mod ls -l
+----------+----------+-------------------------------+----------------------------------------------------------+------------+----------------+
| Name     | Provider | Description                   | URL                                                      | Latest Tag | Latest Release |
+==========+==========+===============================+==========================================================+============+================+
| lambda   | aws      | Terraform AWS Lambda Module   | https://github.com/yhakbar/terraform-aws-lambda-module   | 0.0.1      |                |
+----------+----------+-------------------------------+----------------------------------------------------------+------------+----------------+
| iam-role | aws      | Terraform AWS IAM Role Module | https://github.com/yhakbar/terraform-aws-iam-role-module | 2.1.0      | 2.1.0          |
+----------+----------+-------------------------------+----------------------------------------------------------+------------+----------------+
+-------+--------------+
| Repos | Hidden Repos |
+=======+==============+
| 3     | 1            |
+-------+--------------+
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
