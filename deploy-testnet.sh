#!/bin/sh
# Set up
ROOT_DIR=$PWD
DEPLOY_ACCOUNT="slimebook"
PROJECT_NAME="slimebook"

# Deploy web4 contract
near contract deploy web4.$DEPLOY_ACCOUNT.testnet \
  use-file $ROOT_DIR/web4/target/wasm32-unknown-unknown/release/$PROJECT_NAME.wasm \
  without-init-call \
  network-config testnet \
  sign-with-keychain \
  send

# Delete old mdbook
near contract call-function as-transaction web4.$DEPLOY_ACCOUNT.testnet \
  clear json-args "{}" \
  prepaid-gas '100.0 Tgas' \
  attached-deposit '0 NEAR' \
  sign-as $DEPLOY_ACCOUNT.testnet \
  network-config testnet \
  sign-with-keychain \
  send

# Deploy mdbook
for FILE in $(fd . $ROOT_DIR/book/book/html -t f)
do
  FILE=${FILE#$ROOT_DIR/book/book/html/}
  FILE_CONTENTS=$(cat $ROOT_DIR/book/book/html/$FILE)
  echo "Uploading $FILE..."
  BYTES=$(od -An -vtu1 "$ROOT_DIR/book/book/html/$FILE" | awk '{for(i=1;i<=NF;i++) printf "%s,",$i}' | sed 's/,$/\n/' | sed 's/^/[/' | sed 's/$/]/')
  echo "{\"name\":\"/$FILE\",\"data\":$BYTES}" > args.json
  OUTPUT=$(near contract call-function as-transaction web4.slimebook.testnet \
    upload_file file-args args.json \
    prepaid-gas '300.0 Tgas' \
    attached-deposit '0 NEAR' \
    sign-as $DEPLOY_ACCOUNT.testnet \
    network-config testnet \
    sign-with-keychain \
    send 2>&1)
  rm args.json
  if echo "$OUTPUT" | grep -q "succeeded"; then
    echo "Uploaded $FILE successfully"
  else
    echo "Error uploading $FILE:"
    echo $OUTPUT
    exit 1
  fi
done

# Done
echo ""
echo "Deployed to https://$DEPLOY_ACCOUNT.testnet.page/"
