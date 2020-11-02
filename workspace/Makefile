.PHONY: clean build
build:
	TEXINPUTS=.:../ElegantNote//:${TEXINPUTS} xelatex *.tex

clean:
	-rm ./*.aux
	-rm ./*.out
	-rm ./*.log
