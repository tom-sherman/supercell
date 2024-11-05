# Playbook: Release

To release a new version of Supercell, follow these steps:

1. Commit change to the main branch using the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) format.
2. Check the current version by running `grep 'version = ' Cargo.toml | head -n 1`
3. Update the version in `Cargo.toml` to the version you are releasing.
4. Run the create-release script with the target container, old version and new version: `./create-release.sh ghcr.io/astrenoxcoop/supercell 0.1.0 0.2.0`

