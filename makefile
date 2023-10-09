sbf:
	cargo build-sbf --manifest-path=./contract/Cargo.toml --sbf-out-dir=dist/contract

deploy-remote-local: sbf
	solana config set --url http://127.0.0.1:8899
	solana program deploy dist/contract/contract.so

deploy-remote-dev: sbf
	solana config set --url devnet
	solana program deploy dist/contract/contract.so

deploy-remote-main: sbf
	solana config set --url mainnet-beta
	solana program deploy dist/contract/contract.so

operator:
	cargo build --release --bin operator

server:
	cargo build --release --bin server

debug: 
	set -x 
	echo "hello validator started !"
	make deploy-remote
	sleep 3
	cargo run --bin server
	

all: sbf operator server 

clean: 
	rm -rf dist