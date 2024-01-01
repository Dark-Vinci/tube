#!/bin/sh

echo "Syncing changes in working directory to staging";

cd ..

git add .

echo "Committing changes"

git commit -am "chore: $1"

echo "Pushing to github"

git push -u origin main
