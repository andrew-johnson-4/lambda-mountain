test:
	cargo install --path .
	cargo test -- --nocapture
	lm repl

tex:
	pdflatex calligraphy.tex
