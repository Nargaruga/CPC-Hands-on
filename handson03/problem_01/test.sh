#!/bin/bash

dir="tests"

for i in {0..4}; do
    cargo run < "$dir/input$i.txt" | diff - "$dir/output$i".txt
done
