# txt2pdf

Prints plaintext file to PDF in Courier 12 point font using troff.

Examples:

* Basic usage: `txt2pdf plaintext.txt > plaintext.pdf`
* Line numbers: `txt2pdf -n poem.txt > poem.pdf`
* Diff files: `txt2pdf -n -m -h "C vs Rust" hello_c.S hello_rs.S > hello.S.pdf`
* Columns: `txt2pdf -2 plaintext.txt > plaintext.pdf`

See the examples by running:

```
cd examples
make
open artifacts
```
