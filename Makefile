run:
	cargo build
	cargo run 
	mkdir -p ./target/debug/conf
	cp -a ./conf/. ./target/debug/conf/
	./target/debug/rustbound ../../example
