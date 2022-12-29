#!/bin/bash
case $1 in
    doc)
        echo "Use Cargo & python3"
        cargo doc
        echo "<meta http-equiv='refresh' content='0; url=/leetcode'>" > target/doc/index.html 
        python3 -m http.server 8080 --bind 127.0.0.1 --directory target/doc/
        ;;
    # cargo doc 生成 inde.html jump /leetcode dir
    doc-gi)
        echo "<meta http-equiv='refresh' content='0; url=/leetcode'>" > target/doc/index.html
        ;;
    # daily problem / all problems
    test-dp)
        cargo test --package leetcode --lib -- lc202212::$2 --exact --nocapture
        ;;
    test-ap)
        cargo test --package leetcode --lib -- lc202212ap::$2 --exact --nocapture
        ;;
esac