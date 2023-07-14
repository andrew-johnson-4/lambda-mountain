test:
	cargo install --path .
	cargo test -- --nocapture
	lambda_mountain repl

tex:
	pdflatex calligraphy.tex
