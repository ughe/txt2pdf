TXT2PDF=../txt2pdf

all:
	cd hello && make
	mv hello/*.pdf artifacts
	cd math && ./math.sh
	mv math/*.pdf artifacts
	cd poems && ./poems.sh
	mv poems/*.pdf artifacts
	$(TXT2PDF) -n -h "txt2pdf" ../txt2pdf > artifacts/txt2pdf.pdf

clean:
	rm -f artifacts/*.pdf
	cd hello && make clean
