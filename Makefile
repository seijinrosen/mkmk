test:
	cargo check
	cargo test
	cargo clippy
	# cargo tarpaulin

after-develop-merged:
	git switch main
	git pull --prune
	git branch --delete develop
	git switch --create develop

clean:
	rm -r node_modules/
	rm -r target/

init:
	pnpm install

open:
	gh repo view --web

switch:
	git switch --create develop
