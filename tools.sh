#!/bin/bash

cd `dirname $0`
case $1 in
    # cargo doc 生成 for gitee
    doc)
        cargo doc
        echo "<meta http-equiv='refresh' content='0; url=/kkbt-rust-playground/leetcode'>" > target/doc/index.html 
        ;;
    # cargo doc 生成 并 运行 for 本地
    docr)
        cargo doc
        echo "<meta http-equiv='refresh' content='0; url=/kkbt-rust-playground/leetcode'>" > target/doc/index.html 
        # --- for Windows WSL2 Begin ---
        explorer.exe file://wsl.localhost/Ubuntu/home/kkbt/PlayGround/target/doc/leetcode/index.html
        # --- for Windows WSL2 End ---
        # python3 -m http.server 8080 --bind 127.0.0.1 --directory target/doc/
        ;;
    # cargo doc 生成 inde.html jump /leetcode dir
    doc-gi)
        echo "<meta http-equiv='refresh' content='0; url=/leetcode'>" > target/doc/index.html
        ;;
    # test daily problem / all problems
    test-dp)
        cargo test --package leetcode --lib -- lc202301::$2 --exact --nocapture
        ;;
    test-ap)
        cargo test --package leetcode --lib -- lc202301ap::$2 --exact --nocapture | grep run
        ;;
esac