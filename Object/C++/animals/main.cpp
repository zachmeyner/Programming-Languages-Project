#include <stdio.h>
#include "animal.h"
#include "cat.h"
#include "dog.h"

int main() {
    Cat jeff("Jeff", "Bill");
    Cat fluffy("Fluffy");

    Dog rover("Rover", "Johnathan");
    Dog max("Max");

    printf("%s info:\n", jeff.GetName().c_str());
    jeff.MakeNoise();
    jeff.Identify();
    printf("\n");

    printf("%s info:\n", fluffy.GetName().c_str());
    fluffy.MakeNoise();
    fluffy.Identify();
    printf("\n");

    printf("%s info:\n", rover.GetName().c_str());
    rover.MakeNoise();
    rover.Identify();
    printf("\n");
    
    printf("%s info:\n", max.GetName().c_str());
    max.MakeNoise();
    max.Identify();
    printf("\n");

    printf("Dogs: %d\n", Dog::DogCount());
    printf("\tName: %s\n\tOwner: %s\n", rover.GetName().c_str(), rover.GetOwner().c_str());
    printf("\tName: %s\n\tOwner: %s\n", max.GetName().c_str(), max.GetOwner().c_str());
    printf("\n");
    printf("Cats: %d\n", Cat::CatCount());
    printf("\tName: %s\n\tOwner: %s\n", fluffy.GetName().c_str(), fluffy.GetOwner().c_str());
    printf("\tName: %s\n\tOwner: %s\n", jeff.GetName().c_str(), jeff.GetOwner().c_str());

    return 0;
}
