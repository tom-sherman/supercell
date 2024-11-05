#!/bin/sh

IMAGE=$1
OLD_VERSION=$2
VERSION=$3

command -v git-cliff >/dev/null 2>&1 || { echo >&2 "This script uses https://github.com/orhun/git-cliff, but it is not installed. Aborting."; exit 1; }
command -v gh >/dev/null 2>&1 || { echo >&2 "This script uses https://cli.github.com/, but it is not installed. Aborting."; exit 1; }
GH_PAGER="" gh release list >/dev/null 2>&1 || { echo >&2 "The GitHub cli is not configured. Aborting."; exit 1; }

git checkout main
git pull

git-cliff --tag=${VERSION} --strip=header ${OLD_VERSION}.. > .tmp.release_info
git-cliff -o --tag=${VERSION} --strip=header

sed -i -e "s/^version = \"${OLD_VERSION}\"/version = \"${VERSION}\"/" Cargo.toml

cargo build

git add Cargo.lock Cargo.toml CHANGELOG.md

git commit -m "release: ${VERSION}" -s

git tag -a "${VERSION}" -F .tmp.release_info

git push
git push --tags

gh release create --verify-tag -F .tmp.release_info -t "${VERSION}" ${VERSION}

git pull

git checkout ${VERSION}

docker build --progress=plain -t "${IMAGE}:${VERSION}" .

if test -f "create-release-post.sh"; then
  sh create-release-post.sh "${IMAGE}" "${OLD_VERSION}" "${VERSION}"
fi

git checkout main
