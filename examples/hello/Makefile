TXT2PDF=../../txt2pdf # Local path to executable in the repository
EXES=$(patsubst %.c,%,$(wildcard *.c)) $(patsubst %.rs,%,$(wildcard *.rs))
OBJS=$(patsubst %.c,%.o,$(wildcard *.c)) $(patsubst %.rs,%.o,$(wildcard *.rs))
ASMS=$(patsubst %.c,%.S,$(wildcard *.c)) $(patsubst %.rs,%.S,$(wildcard *.rs))

CC=gcc
CFLAGS=-O3 -Wall -Wextra -fno-common
RSC=rustc
RSFLAGS=$\
  -C panic=abort $\
  -C opt-level=3 $\
  -C link-args=-lc $\

# Rust LD Flags (macOS)
LDFLAGS_MACOS=$\
  -macosx_version_min 10.8 $\
  -syslibroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk $\
  -lSystem /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/*/lib/darwin/libclang_rt.osx.a $\

# C
%_c.S: %_c.c
	$(CC) $(CFLAGS) -o $@ -S $^
%_c.o: %_c.S
	$(CC) $(CFLAGS) -o $@ -c $^
%_c: %_c.o
	$(CC) $(CFLAGS) -o $@ $^

# Rust
%_rs.S: %_rs.rs
	$(RSC) $(RSFLAGS) -o $@ --emit=asm $^
%_rs.o: %_rs.rs
	$(RSC) $(RSFLAGS) -o $@ --emit=obj $^
## TODO: Uncomment for hello_rs.rs VERSION 1
#%_rs: %_rs.o
#	ld $(LDFLAGS_MACOS) -o $@ $^
%_rs: %_rs.rs
	$(RSC) $(RSFLAGS) -o $@ $^

# Example exists to demonstrate txt2pdf
hello_c_vs_rs.S.pdf: hello_c.S hello_rs.S
	$(TXT2PDF) -n -m -h '"C vs Rust: hello world x86 -O3 (truncated 30 chars)"' hello_c.S hello_rs.S > $@

all: $(ASMS) $(OBJS) $(EXES) hello_c_vs_rs.S.pdf
clean:
	rm -f *.o *.bc *.S
	rm -f $(EXES) $(PDFDST) hello_c_vs_rs.S.pdf
