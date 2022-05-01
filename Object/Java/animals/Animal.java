import java.util.Optional;

abstract class Animal {
    private int id;
    private String name;
    private Optional<String> owner;

    public Animal(int anID, String anName) {
        id = anID;
        name = anName;
        owner = Optional.empty();
    }

    public Animal(int anID, String anName, String ownName) {
        id = anID;
        name = anName;
        owner = Optional.ofNullable(ownName);
    }

    public int ID(){
        return id;
    }
    public String GetName(){
        return name;
    }
    public String GetOwner(){
        return owner.orElse("No owner");
    }
    public void SetName(String newName){
        name = newName;
    }
    public void SetOwner(String newOwner){
        owner = Optional.ofNullable(newOwner);
    }
    public void RemoveOwner() {
        owner = Optional.empty();
    }

    abstract void MakeNoise();
    public void Identify(){
        System.out.println("My name is " + name + ".");
        if (owner.isEmpty()) {
            System.out.println("I have no owner.\nMy id is " + id +".\n");
        } else {
            System.out.println("My owner is " + owner.get() +".\nMy id is " + id +".\n");
        }
    }

}

