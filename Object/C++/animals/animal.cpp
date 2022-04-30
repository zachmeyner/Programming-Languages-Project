#include <string>
#include <optional>
#include "animal.h"
#include <stdio.h>

Animal::Animal(int anID, std::string anName): id(anID), name(anName) {
    owner = std::nullopt;
}

Animal::Animal(int anID, std::string anName, std::string anOwner): id(anID), name(anName), owner(anOwner) {}

int Animal::ID() const {
    return id;
}

std::string Animal::GetName() const {
    return name;
}

std::string Animal::GetOwner() const {
    return owner.value_or("No owner");
}

void Animal::SetName(std::string newName) {
    name = newName;
}

void Animal::Identify() const {
    printf("My name is %s.\n", name.c_str());
    if (owner == std::nullopt) {
        printf("I have no owner.\nMy id is %d.\n", id);
    } else {
        printf("My owner is %s.\nMy id is %d.\n", owner.value().c_str(), id);
    }
}