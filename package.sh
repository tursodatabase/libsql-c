
set -xe

target="${1:?"explit target is required"}"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    manifest="$(powershell -Command "[System.IO.Path]::GetFullPath('$2')")"
else
    manifest="$(realpath ${2:?"explicit manifest path is required"})"
fi
extra="${3:-}"

artifacts=$(
  cargo build \
    --message-format json \
    --target $target \
    --manifest-path $manifest \
    $extra \
    | jq -r --arg manifest "$manifest" '
          select(.manifest_path == $manifest)
          | .filenames[]
      '
)

artifacts_dir="$(echo "$artifacts" | sed '1s|/[^/]*$||;q')"
type="$(echo "$artifacts_dir" | sed 's|.*/||')"

mkdir -p dist/

echo "$artifacts" | zip -j@ dist/$target-$type.zip

zip -j dist/$target-$type.zip "$(dirname $manifest)/libsql.h"
