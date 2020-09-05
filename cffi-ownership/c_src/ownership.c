#include <stdlib.h>
#include <stdio.h>

int * make_memory() {
    int *i;

    i = malloc(sizeof(int));
    *i = 2;

    return i;
}

void take_ownership(int *i, void(*dtor)(int *)) {
    printf("got %d\n", *i);
    // C のコードでメモリを解放する
    // Rust で用意した値は Rust から貰ったデストラクタで解放する
    dtor(i);
}
