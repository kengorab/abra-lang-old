#include "gc.h"

void __mod_entry();

int main() {
    GC_init();

    __mod_entry();
    return 0;
}
