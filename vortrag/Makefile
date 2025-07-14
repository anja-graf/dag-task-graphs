JOB = vortrag
PDFLATEX = pdflatex
PDFOPTS = -halt-on-error -file-line-error -interaction nonstopmode

$(JOB).pdf: $(JOB).tex
	mkdir -p build
	$(PDFLATEX) $(PDFOPTS) -output-directory build $<
	cp build/$@ $@

run:
	xdg-open $(JOB).pdf 2>/dev/null || \
		zathura $(JOB).pdf 2>/dev/null || \
		evince $(JOB).pdf 2>/dev/null &

clean:
	rm -f build/*.toc build/*.tod build/*.aux build/*.bbl build/*.blg \
		build/*.log build/*.out build/*.run.xml build/*.bcf build/*.lof \
		build/*.nav build/*.snm build/*.vrb \
		build/*.lot build/*.makefile build/*.figlist
	rm -f build/tikz/*.pdf build/tikz/*.log build/tikz/*.dpth build/tikz/*.md5
	rm -f build/*.pdf
	-rmdir -p build

.PHONY: clean 

