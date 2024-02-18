#!/bin/sh

cd ..

echo "Syncing changes in working directory to staging";

# format all the DOCS
directory="./BACKEND"

# Loop over each directory within the specified directory
for dir in "$directory"/*/; do
    # Extract and print the name of the directory
    dirname=$(basename "$dir")

    cd $dir && cargo fmt

    cd ..
done

#cd into ui and run lint
cd ../UI

npm run format
npm run lint

# cd back into the project and commit
cd ..

git add .

echo "🤓🤓Committing changes🤓🤓"

git commit -am "chore: $1"

echo "🤭🤭Pushing to github🤭🤭"

git push -u origin main

echo "🚀🚀Pushed to github 🚀🚀"
