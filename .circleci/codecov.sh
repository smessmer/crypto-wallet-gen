#!/bin/bash

set -e

# Build and find all test executables
REPORT=$(cargo test --no-run --message-format=json | jq -r "select(.profile.test == true) | .filenames[]")

echo Generating coverage for files
echo $REPORT

for file in $REPORT; do
    mkdir -p "target/cov/$(basename $file)"
    /opt/kcov/bin/kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"
done

wget -O - -q "https://codecov.io/bash" > .codecov
chmod +x .codecov
./.codecov
echo "Uploaded code coverage"
