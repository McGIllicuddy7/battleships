#include <stdio.h>
#define USING_BLIBC
#include "../bridget-libc/blibc.h"
#include <raylib.h>

int main(void){
     SetTraceLogLevel(LOG_ERROR);
     InitWindow(640, 480, "hello window");
     while (!WindowShouldClose())
     {
          BeginDrawing();
          ClearBackground(BLACK);
          DrawText("hello world!", 10, 10, 18, GREEN);
          EndDrawing();
     }
     
}
