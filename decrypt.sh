#!/usr/bin/env bash
set -euo pipefail

ENCRYPTED_FILES=`find . -type f -regex ".*\.enc\(\.yaml\|\.json\)?\$"`
for FILE in ${ENCRYPTED_FILES}; do
  DECRYPTED_FILE=`echo "$FILE" | sed 's/.enc././g'`
  echo "Decrypting $FILE"
  sops --decrypt $FILE > $DECRYPTED_FILE
done

