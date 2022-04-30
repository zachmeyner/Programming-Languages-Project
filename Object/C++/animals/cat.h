#ifndef ZACH_CAT_411_
#define ZACH_CAT_411_

#include "animal.h"

class Cat : public Animal {
    private:
        static inline int number = 0;

    public:
        Cat(std::string myName, std::string ownName);
        Cat(std::string myName);
        void Identify() const override;
        void MakeNoise() const override;

        static int CatCount();
};


#endif