public class Cat extends Animal {
    private static int number = 0;

    public Cat(String anName) {
        super(++number, anName);
    }

    public Cat(String anName, String anOwner) {
        super(++number, anName, anOwner);
    }

    public void MakeNoise() {
        System.out.println("Meow.");
    }

    public void Identify() {
        System.out.println("I am a cat.");
        super.Identify();
    }

    public static int CatCount() {
        return number;
    }
}
