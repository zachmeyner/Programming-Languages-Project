#include "dog.h"

Dog::Dog(std::string myName, std::string ownName): Animal(++number, myName, ownName) {}
Dog::Dog(std::string myName): Animal(++number, myName) {}

int Dog::DogCount() {
    return number;
}

void Dog::Identify() const {
    printf("A dog!\n");
    Animal::Identify();
}

void Dog::MakeNoise() const {
    printf("Woof!\n");
}

