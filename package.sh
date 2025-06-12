
set -xe

target="${1:?"explit target is required"}"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    manifest="$(powershell -Command "[System.IO.Path]::GetFullPath('$2')")"
else
    manifest="$(realpath ${2:?"explicit manifest path is required"})"
fi

for extra in '' --release; do
  artifacts=$(
    cargo build \
      --message-format json \
      --target $target \
      --manifest-path $manifest \
      $extra
  )

  artifacts=$(
    echo "$artifacts" | jq -r --arg manifest "$manifest" '
      select(.manifest_path == $manifest)
        | .filenames[]
    '
  )
  artifacts=$(echo "$artifacts" | tr -d '\r')

  artifacts_dir="$(echo "$artifacts" | sed '1s|[/\\][^/\\]*$||;q')"
  type="$(echo "$artifacts_dir" | sed 's|.*[/\\]||')"

  mkdir -p dist/
  mkdir -p dist/a

  cp $artifacts "$(dirname $manifest)/libsql.h" dist/a
  cd dist/a

  if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
		7z a -tzip ../$target-$type.zip *;
	else
		zip -r ../$target-$type.zip *;
	fi;

  cd ../..
  rm -rf dist/a/
done

