public class Main {
    public static void main(String args[]) {
        Dog rover = new Dog("Rover", "Johnathan");
        Dog max = new Dog("Rover");
        Cat jeff = new Cat("Jeff", "Bill");
        Cat fluffy = new Cat("Fluffy");

        System.out.printf("%s info:\n", jeff.GetName());
        jeff.MakeNoise();
        jeff.Identify();
        System.out.printf("\n");

        System.out.printf("%s info:\n", fluffy.GetName());
        fluffy.MakeNoise();
        fluffy.Identify();
        System.out.printf("\n");

        System.out.printf("%s info:\n", rover.GetName());
        rover.MakeNoise();
        rover.Identify();
        System.out.printf("\n");
        
        System.out.printf("%s info:\n", max.GetName());
        max.MakeNoise();
        max.Identify();
        System.out.printf("\n");

        System.out.printf("Dogs: %d\n", Dog.DogCount());
        System.out.printf("\tName: %s\n\tOwner: %s\n", rover.GetName(), rover.GetOwner());
        System.out.printf("\tName: %s\n\tOwner: %s\n", max.GetName(), max.GetOwner());
        System.out.printf("Swapping owner status.\n");

        rover.RemoveOwner();
        max.SetOwner("Johnathan");
        System.out.printf("\tName: %s\n\tOwner: %s\n", rover.GetName(), rover.GetOwner());
        System.out.printf("\tName: %s\n\tOwner: %s\n", max.GetName(), max.GetOwner());
        System.out.printf("\n");

        System.out.printf("Cats: %d\n", Cat.CatCount());
        System.out.printf("\tName: %s\n\tOwner: %s\n", fluffy.GetName(), fluffy.GetOwner());
        System.out.printf("\tName: %s\n\tOwner: %s\n", jeff.GetName(), jeff.GetOwner());
        System.out.printf("Swapping owner status.\n");

        jeff.RemoveOwner();
        fluffy.SetOwner("Johnathan");
        System.out.printf("\tName: %s\n\tOwner: %s\n", fluffy.GetName(), fluffy.GetOwner());
        System.out.printf("\tName: %s\n\tOwner: %s\n", jeff.GetName(), jeff.GetOwner());
    }
}   