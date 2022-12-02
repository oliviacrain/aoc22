set dotenv-load

aoc_session := env_var('AOC_SESSION')

setup DAY: && (fetch DAY)
    cargo new --bin "solutions/day{{DAY}}a"
    cargo new --bin "solutions/day{{DAY}}b"
    cp template.rs "solutions/day{{DAY}}a/src/main.rs"
    cp template.rs "solutions/day{{DAY}}b/src/main.rs"

fetch DAY:
    @curl -s -o input.txt 'https://adventofcode.com/2022/day/{{DAY}}/input' --cookie "session={{aoc_session}}" --user-agent "curl-7.85-oliviacrain"
    cp input.txt solutions/day{{DAY}}a/input.txt
    mv input.txt solutions/day{{DAY}}b/input.txt
