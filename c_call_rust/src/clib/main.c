#include <stdio.h>
#include "Point.h"

int main(void) {
    Point obj = {
        .x = 0,
        .y = 0
    };
    print_point(1, 3, &obj);
}