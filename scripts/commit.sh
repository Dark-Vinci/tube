#!/bin/sh

cd ..

echo "Syncing changes in working directory to staging";

directory="./BACKEND"

# Loop over each directory within the specified directory
for dir in "$directory"/*/; do
    # Extract and print the name of the directory
    dirname=$(basename "$dir")
    echo "$dirname"
    cd "$dir" && cargo fmt && cd directory;
    # echo "$dirname"
done

git add .

echo "🤓🤓Committing changes🤓🤓"

git commit -am "chore: $1"

echo "🤭🤭Pushing to github🤭🤭"

git push -u origin main

echo "🚀🚀Pushed to github 🚀🚀"
