#!/usr/bin/env sh

# Get project name from package.json
function project_name() {
python3 <<END
import json
with open("package.json", "r") as read_file:
    data = json.load(read_file)
    key = list(data["dependencies"].keys())[0]
    print(key)
END
}

rm -rf ../pkg/
wasm-pack build

# Replace the node module with the new wasm files
rm -rf node_modules/$(project_name)/
npm install

npm run start

