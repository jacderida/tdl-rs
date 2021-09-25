#!/usr/bin/env bash

version=$1
if [[ -z "$version" ]]; then
    echo "You must supply a version number for tdl."
    exit 1
fi

# The single quotes around EOF is to stop attempted variable and backtick expansion.
read -r -d '' release_description << 'EOF'
# Terminal Doom Launcher __VERSION__

## Changes

__CHANGELOG__

## SHA-256 checksums:
```
Linux
__TAR_LINUX_CHECKSUM__

macOS
__TAR_MACOS_CHECKSUM__

Windows
__TAR_WIN_CHECKSUM__
```
EOF

tar_linux_checksum=$(sha256sum \
    "../../release/tdl-$version-x86_64-unknown-linux.tar.gz" | \
    awk '{ print $1 }')
tar_macos_checksum=$(sha256sum \
    "../../release/tdl-$version-x86_64-apple-darwin.tar.gz" | \
    awk '{ print $1 }')
tar_win_checksum=$(sha256sum \
    "../../release/tdl-$version-x86_64-pc-windows-msvc.tar.gz" | \
    awk '{ print $1 }')

release_description=$(sed "s/__VERSION__/$version/g" <<< "$release_description")
release_description=$(sed "s/__TAR_LINUX_CHECKSUM__/$tar_linux_checksum/g" <<< "$release_description")
release_description=$(sed "s/__TAR_MACOS_CHECKSUM__/$tar_macos_checksum/g" <<< "$release_description")
release_description=$(sed "s/__TAR_WIN_CHECKSUM__/$tar_win_checksum/g" <<< "$release_description")
echo "$release_description" > ../../release_description.txt

./insert_change_log.py "$version"
