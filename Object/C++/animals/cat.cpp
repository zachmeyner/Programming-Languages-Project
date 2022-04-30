#include <stdio.h>
#include "cat.h"

Cat::Cat(std::string myName, std::string ownName): Animal(number+=1, myName, ownName){}
Cat::Cat(std::string myName): Animal(number+= 1, myName){}

int Cat::CatCount() {
    return number;
}

void Cat::Identify() const {
    printf("A cat\n");
    Animal::Identify();
}

void Cat::MakeNoise() const {
    printf("Meow\n");
}