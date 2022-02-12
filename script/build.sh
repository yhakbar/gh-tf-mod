#!/usr/bin/env bash
echo "TODO implement this script."
echo "It should build binaries in dist/<platform>-<arch>[.exe] as needed."
# exit 1

targets=$(cat <<EOT
{
    "targets": [
        {
            "arch": "aarch64",
            "platform": "apple-darwin"
        },
        {
            "arch": "aarch64",
            "platform": "unknown-linux-gnu"
        },
        {
            "arch": "x86_64",
            "platform": "apple-darwin"
        },
        {
            "arch": "x86_64",
            "platform": "unknown-linux-gnu"
        }
    ]
}
EOT
)

for target in $(echo "$targets" | jq -cr '.targets'); do
    arch="$(echo "$target" | jq -r '.arch')"
    platform="$(echo "$target" | jq -r '.platform')"
    echo "Building for target $arch-$platform"
    rustup target add "$arch-$platform"
    cargo build --release --target "$arch-$platform"
done
