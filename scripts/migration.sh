#!/bin/sh

case $1 in
  "auth")
    echo "about to cd into auth service to generate migration"
    # shellcheck disable=SC2164
    cd ../BACKEND/auth/src
    sea-orm-cli migrate generate "$2";;

  "reactions")
    echo "about to cd into reactions service to generate migration"
    # shellcheck disable=SC2164
    cd ../BACKEND/reactions/src
    sea-orm-cli migrate generate "$2";;

  "posts")
    echo "about to cd into posts service to generate migration"
    # shellcheck disable=SC2164
    cd ../BACKEND/posts/src
    sea-orm-cli migrate generate "$2";;

  "utils")
    echo "about to cd into utils service to generate migration"
    # shellcheck disable=SC2164
    cd ../BACKEND/utils/src
    sea-orm-cli migrate generate "$2";;

  *)
    echo "No known service was chosen";;
esac
