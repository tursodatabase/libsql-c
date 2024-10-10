CC = gcc
LDFLAGS = -L../../target/release -Wl,-rpath,../../target/release
LIBS = -llibsql

TARGET = example

all: $(TARGET)

$(TARGET): example.c
	$(CC) $(LDFLAGS) -o $@ $< $(LIBS)

clean:
	rm -f $(TARGET)

.PHONY: all clean
