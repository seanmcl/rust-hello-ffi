#include <stdint.h>
#include <stdio.h>

int32_t length(const char* x, const char* y);

int main() {
    printf("%d\n", length("foo", "bar"));
    return 0;
}
