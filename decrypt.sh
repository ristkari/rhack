#!/usr/bin/env bash
set -euo pipefail

ENCRYPTED_FILES=`find secrets -type f -name "*.enc.*"`
for FILE in ${ENCRYPTED_FILES}; do
  DECRYPTED_FILE=`echo "$FILE" | sed 's/.enc././g'`
  echo "Decrypting $FILE"
  sops --decrypt $FILE > $DECRYPTED_FILE
done

