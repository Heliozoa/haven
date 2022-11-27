#!/bin/sh

(
    watchexec \
        --workdir "./backend" \
        --watch "./backend/src/api.rs" \
        --watch "./backend/src/bin/export-elm.rs" \
        --restart \
        "cargo run --bin export-elm" &

    watchexec \
        --workdir "./backend" \
        --watch "./backend/src" \
        --restart \
        "cargo run" &

    cd frontend &&
        elm-spa server &
)

sleep infinity
