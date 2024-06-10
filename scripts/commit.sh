#!/bin/sh
# -z, -n, -r, -f, -d

set -x
set -eo pipefail

if [[ -z $1 ]]; then
    echo >&2 "no commit message"
    exit
fi

echo >&1 "BEGIN COMMIT"

cd ..

echo "Syncing changes in working directory to staging";

# format all the DOCS
directory="./BACKEND"

# Loop over each directory within the specified directory
for dir in "$directory"/*/; do
    dirname=$(basename "$dir")
    
    if [[ "$dirname" -eq "target" ]]; then
        continue
    fi

    cd $dir && cargo fmt

    cd ..
done

#cd into ui and run lint
cd UI

npm run format
npm run lint

cd ../dashboard

npm run format
npm run lint

# cd back into the project and commit
cd ..

git add .

echo "🤓🤓Committing changes🤓🤓\n"

git commit -am "chore: $1"

echo "🤭🤭Pushing to github🤭🤭\n"

git push -u origin main

echo "🚀🚀Pushed to github 🚀🚀\n"
