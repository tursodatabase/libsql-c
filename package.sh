
set -xe

target="${1:?"explit target is required"}"
manifest="$(realpath ${2:?"explicit manifest path is required"})"
extra="${3:-}"

artifacts=$(
  cargo build \
    --message-format json \
    --target $target \
    --manifest-path $manifest \
    $extra \
    | jq -r "
          select(.manifest_path == \"${manifest}\")
          | .filenames[]
      "
)

artifacts_dir="$(echo "$artifacts" | sed '1s|/[^/]*$||;q')"
type="$(echo "$artifacts_dir" | sed 's|.*/||')"

echo $artifacts_dir
echo "$(echo "$artifacts" | sed 's|.*/||')"

mkdir -p dist/

echo "$artifacts" \
  | sed 's|.*/||' \
  | tar -cf dist/$target-$type.tar -C $artifacts_dir -T -

tar -rf dist/$target-$type.tar -C $(dirname $manifest) libsql.h

gzip dist/$target-$type.tar
