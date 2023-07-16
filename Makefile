test:
	cargo install --path .
	cargo test -- --nocapture
	lm examples/history.lm

tex:
	pdflatex calligraphy.tex
