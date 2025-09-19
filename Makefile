# Hybrid makefile for both Windows and POSIX like targets.
# Always specify the OS target when using this!

# This assumes GCC, which should be prebuilt on all POSIX systems (x64)
posix:
	@echo "Buidling POSIX target..."
	CC := gcc
	CFLAGS := -Wall -Wextra -Wpedantic -std=c99
	LIBS := -lm

# This assumes MSVC, which comes with Visual Studio 2019 and later (x64)
windows:
	@echo "Building Windows target..."
	CC := cl
	CFLAGS := /W3 /STD:c99