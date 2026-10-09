
# Prod ONLY
#CXX = g++
#CXXFLAGS = -std=c++20 -O3 -Wall -Wextra -march=native
#
#main: main.o
#	$(CXX) $(CXXFLAGS) main.o -o main
#
#main.o: main.cpp
#	$(CXX) $(CXXFLAGS) -c main.cpp
#
#clean:
#	rm -f main main.o

CXX = g++
CXXFLAGS = -std=c++20 -Wall -Wextra -march=native

BUILD ?= debug

ifeq ($(BUILD), release)
    CXXFLAGS += -O3 -DNDEBUG
else
    CXXFLAGS += -Og -g -fno-omit-frame-pointer -fsanitize=address,undefined
    LDFLAGS += -fsanitize=address,undefined
endif

main: main.o
	$(CXX) $(CXXFLAGS) main.o -o main $(LDFLAGS)

main.o: main.cpp
	$(CXX) $(CXXFLAGS) -c main.cpp

clean:
	rm -f main main.o