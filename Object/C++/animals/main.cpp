#include <iostream>
#include "animal.h"
#include "cat.h"

int main() {
    Cat jeff("Jeff", "Bill");

    jeff.MakeNoise();
    jeff.Identify();

    return 0;
}
