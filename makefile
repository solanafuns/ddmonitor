sbf:
	cargo build-sbf --manifest-path=./contract/Cargo.toml --sbf-out-dir=dist/contract
	
deploy-remote: sbf
	solana program deploy dist/contract/ddmonitor.so

operator:
	cargo build --release --bin operator

server:
	cargo build --release --bin server

debug: server operator
	set -x 
	solana-test-validator -r > /dev/null 2>&1 & 
	pgrep solana-test-validator > validator.pid
	echo "hello validator started !"
	sleep 10
	kill `cat validator.pid` && rm validator.pid

all: sbf operator server 

clean: 
	rm -rf dist