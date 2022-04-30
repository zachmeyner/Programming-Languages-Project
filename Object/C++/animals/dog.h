#ifndef ZACH_DOG_411_
#define ZACH_DOG_411_

#include "animal.h"

class Dog : public Animal {
    private:
        static inline int number = 0;
    
    public:
        Dog(std::string myName, std::string ownName);
        Dog(std::string myName);
        void Identify() const override;
        void MakeNoise() const override;

        static int DogCount();
};

#endif