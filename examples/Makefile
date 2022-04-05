all:
	cd hello && make
	mv hello/*.pdf artifacts
	cd math && ./math.sh
	mv math/*.pdf artifacts
	cd poems && ./poems.sh
	mv poems/*.pdf artifacts

clean:
	rm -f artifacts/*.pdf
	cd hello && make clean
