#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

makecurl() {
   echo $1
   curl -v "$1" 2>&1 | gum style --foreground $2
}

#https://github.com/bats-core/bats-core
main() {
     #make sure binary is built
     cargo build
     #start server
     cargo run &
     # this should return 404 not found error
     sleep 5s
     makecurl http://localhost:4221/abcdefg 2
     #this should return 200 ok
     sleep 5s
     makecurl http://localhost:4221  1

}

main "$@"
