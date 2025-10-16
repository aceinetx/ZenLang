while true; do
		echo running 
		./target/debug/zenlang-cli a.zen
    if [ $? -ne 0 ]; then
        break
    fi
		echo continuing...
done
