test:
	@cargo fmt -q
	@cargo check -q

clip:
	@cargo clippy -q

