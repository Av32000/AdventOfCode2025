#!/bin/bash

README="README.md"
DAYS_DIR="src/days"

if [ ! -f "$README" ]; then
    echo "README.md not found!"
    exit 1
fi

TABLE_CONTENT=""

for i in $(seq -w 1 25); do
    DAY_DIR="$DAYS_DIR/$i"
    
    if [ -d "$DAY_DIR" ]; then
        ROW="| [Day $i](src/days/$i) |"
        
        if [ -f "$DAY_DIR/part1.rs" ]; then
            ROW="$ROW [Part 1](src/days/$i/part1.rs) |"
        else
            ROW="$ROW Not Available |"
        fi
        
        if [ -f "$DAY_DIR/part2.rs" ]; then
            ROW="$ROW [Part 2](src/days/$i/part2.rs) |"
        else
            ROW="$ROW Not Available |"
        fi
        
        TABLE_CONTENT="${TABLE_CONTENT}${ROW}\n"
    fi
done

HEADER="| :---: | :---: | :---: |"

TEMP_README=$(mktemp)

sed '/| :---: | :---: | :---: |/q' "$README" > "$TEMP_README"

LAST_LINE=$(tail -n 1 "$TEMP_README")
if [[ "$LAST_LINE" != *"$HEADER"* ]]; then
    echo "Table header not found in README.md or it is not the last thing processed."
    rm "$TEMP_README"
    exit 1
fi

printf "$TABLE_CONTENT" >> "$TEMP_README"

mv "$TEMP_README" "$README"
echo "README.md updated."
