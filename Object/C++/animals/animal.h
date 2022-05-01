#ifndef ZACH_ANIMAL_411_
#define ZACH_ANIMAL_411_

#include <string>
#include <optional>

class Animal {
    private:
        const int id;
        std::string name;
        std::optional<std::string> owner;

    public: 
        Animal(int anID, std::string anName);
        Animal(int anID, std::string anName, std::string anOwner);

        virtual ~Animal() = default;

        virtual int ID() const;
        virtual std::string GetName() const;
        virtual std::string GetOwner() const;
        virtual void SetName(std::string newName);
        virtual void SetOwner(std::string newOwn);
        virtual void RemoveOwner();

        virtual void MakeNoise() const = 0;
        virtual void Identify() const;
};


#endif