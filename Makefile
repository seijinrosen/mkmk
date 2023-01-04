test:
	cargo check
	cargo test

after-develop-merged:
	git switch main
	git pull --prune
	git branch --delete develop
	git switch --create develop

init:
	pnpm install

open:
	gh repo view --web

switch:
	git switch --create develop
