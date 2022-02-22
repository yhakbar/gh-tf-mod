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
$ gh tf-mod ls provider-foo
...
# or
$ gh tf-mod ls -p provider foo
...
```

## List

## List Terraform Modules

Use the `ls` subcommand without any target to list all modules in a GitHub organization.

```bash
$ gh tf-mod ls
+------+----------+
| Name | Provider |
+======+==========+
| foo  | provider |
+------+----------+
| bar  | provider |
+------+----------+
+-------+--------------+
| Repos | Hidden Repos |
+=======+==============+
| 3     | 1            |
+-------+--------------+
```

Use the `-l|--long` flag to see all the optional data that is available:

```bash
$ gh tf-mod ls -l
+------+----------+-------------------------------+------------------------------------------------------+------------+----------------+
| Name | Provider | Description                   | URL                                                  | Latest Tag | Latest Release |
+======+==========+===============================+======================================================+============+================+
| foo  | provider | Terraform Provider Foo Module | https://github.com/org/terraform-provider-foo-module | 0.0.1      |                |
+------+----------+-------------------------------+------------------------------------------------------+------------+----------------+
| bar  | provider | Terraform Provider Bar Module | https://github.com/org/terraform-provider-bar-module | 2.1.0      | 2.1.0          |
+------+----------+-------------------------------+------------------------------------------------------+------------+----------------+
+-------+--------------+
| Repos | Hidden Repos |
+=======+==============+
| 3     | 1            |
+-------+--------------+
```

## List Info for a Terraform Module

```bash
$ gh tf-mod ls provider-bar
+------+----------+------------+----------------+
| Name | Provider | Latest Tag | Latest Release |
+======+==========+============+================+
| bar  | provider | 2.1.0      | 2.1.0          |
+------+----------+------------+----------------+
```

## List Extra Info for a Terraform Module

```bash
$ gh tf-mod ls provider-bar -l
+------+----------+-------------------------------+------------------------------------------------------+------------+----------------+
| Name | Provider | Description                   | URL                                                  | Latest Tag | Latest Release |
+======+==========+===============================+======================================================+============+================+
| bar  | provider | Terraform Provider Bar Module | https://github.com/org/terraform-provider-bar-module | 2.1.0      | 2.1.0          |
+------+----------+-------------------------------+------------------------------------------------------+------------+----------------+
+-------+-----------------------------------------------------------------+
| Tag   | URL                                                             |
+=======+=================================================================+
| 2.1.0 | https://github.com/org/terraform-provider-bar-module/commit/abc |
+-------+-----------------------------------------------------------------+
| 2.0.0 | https://github.com/org/terraform-provider-bar-module/commit/xyz |
+-------+-----------------------------------------------------------------+
| 1.0.0 | https://github.com/org/terraform-provider-bar-module/commit/123 |
+-------+-----------------------------------------------------------------+
+------------+------------+
| Tags Total | End Cursor |
+============+============+
| 7          | Mw         |
+------------+------------+
+---------+-------+-------------------------------------------------------------------------+
| Release | Tag   | URL                                                                     |
+=========+=======+=========================================================================+
| 2.1.0   | 2.1.0 | https://github.com/org/terraform-provider-bar-module/releases/tag/2.1.0 |
+---------+-------+-------------------------------------------------------------------------+
| 2.0.0   | 2.0.0 | https://github.com/org/terraform-provider-bar-module/releases/tag/2.0.0 |
+---------+-------+-------------------------------------------------------------------------+
| 1.0.0   | 1.0.0 | https://github.com/org/terraform-provider-bar-module/releases/tag/1.0.0 |
+---------+-------+-------------------------------------------------------------------------+
+----------------+------------+
| Releases Total | End Cursor |
+================+============+
| 7              | xyz        |
+----------------+------------+
```
