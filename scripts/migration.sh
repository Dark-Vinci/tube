#!/bin/sh

case $1 in
  "auth")
    echo "about to cd into auth service to generate migration"
    cd ../BACKEND/auth/src
    sea-orm-cli migrate generate "$2"
   ;;

  "reactions")
    echo "about to cd into reactions service to generate migration"
    cd ../BACKEND/reactions/src
    sea-orm-cli migrate generate "$2"
    ;;

  "posts")
    echo "about to cd into posts service to generate migration"
    cd ../BACKEND/posts/src
    sea-orm-cli migrate generate "$2"
    ;;

  *)
    echo "No known service was chosen"
    ;;
esac
