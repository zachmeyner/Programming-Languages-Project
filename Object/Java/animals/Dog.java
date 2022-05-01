public class Dog extends Animal {
    private static int number = 0;

    public Dog(String anName) {
        super(++number, anName);
    }

    public Dog(String anName, String anOwner) {
        super(++number, anName, anOwner);
    }

    public void MakeNoise() {
        System.out.println("Woof!");
    }

    public void Identify() {
        System.out.println("I am a dog!");
        super.Identify();
    }

    public static int DogCount() {
        return number;
    }
}
