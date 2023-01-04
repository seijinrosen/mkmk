test:
	cargo check
	cargo test

init:
	pnpm install

open:
	gh repo view --web

switch:
	git switch --create develop
