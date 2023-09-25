contract:
	cargo build-sbf --manifest-path=./contract/Cargo.toml --sbf-out-dir=dist/contract

deploy-remote: contract
	solana program deploy dist/contract/ddmonitor.so

operator:
	cargo build --release --bin operator

server:
	cargo build --release --bin server

debug:
	set -x 
	solana-test-validator -r > /dev/null 2>&1 & 
	pgrep solana-test-validator > validator.pid
	echo "hello validator started !"
	sleep 3 
	kill `cat validator.pid` && rm validator.pid

all: contract operator server 

clean: 
	rm -rf dist