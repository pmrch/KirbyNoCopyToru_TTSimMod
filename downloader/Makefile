CC = clang
CC_WIN = cl.exe

CFLAGS = -std=c17 -march=native -ffast-math -O3 -MMD -MP -Iinclude -Weverything -Werror -Wno-unsafe-buffer-usage -Wno-used-but-marked-unused
CFLAGS_WIN = /std:c17 /arch:AVX2 /O2 /WX /Wall /Iinclude /MT /GL

LDFLAGS = -flto -static -lcurl -lyyjson
LDFLAGS_WIN = /LTCG /OPT:REF /OPT:ICF /SUBSYSTEM:CONSOLE

SRC = src/main.c
OBJ = build/main.o

TARGET = downloader
TARGET_WIN = downloader.exe

linux:
	@mkdir -p build
	$(CC) $(CFLAGS) -c $(SRC) -o $(OBJ)
	$(CC) $(LDFLAGS) $(OBJ) -o $(TARGET)
	@rm -rf $(OBJ)