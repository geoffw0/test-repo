CXX ?= c++
CXXFLAGS ?= -std=c++11 -Wall -Wextra -pedantic

.PHONY: all run clean

all: build/main

build/main: src/main.cpp
	mkdir -p build
	$(CXX) $(CXXFLAGS) -o $@ $<

run: build/main
	./build/main

clean:
	rm -rf build
