#include <stdio.h>
#include "cat.h"

Cat::Cat(std::string myName, std::string ownName): Animal(number++, myName, ownName){}
Cat::Cat(std::string myName): Animal(number++, myName){}

int Cat::CatCount() {
    return number;
}

void Cat::Identify() const {
    printf("A cat.\n");
    Animal::Identify();
}

void Cat::MakeNoise() const {
    printf("Meow\n");
}