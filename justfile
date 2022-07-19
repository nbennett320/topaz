test:
	cargo build --release
	rspec spec/run.rb
