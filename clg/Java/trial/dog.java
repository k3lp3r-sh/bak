public class dog extends animal {
  String sound;
  int legs;

  dog(String soun, int l) {
    // this();
    super("mullai");
    sound = soun;
    legs = l;
  }

  dog() {
    System.out.println("senthil");
  }

  public void sound() {
    System.out.println(sound);
  }

  public void colour() {
    System.out.println("whiteieee");
    this.colour("white");
  }

  public void colour(String colour) {
    System.out.println(colour);
  }

  public void name(String name) {
    System.out.println(name);
    super.name();
  }

  public void legs() {
    System.out.println(legs);
  }

  public static void main(String[] arg) {
    dog d = new dog("cry", 2);
    d.sound();
    d.colour();
    d.name("nitini");
    d.legs();
    d.name();
  }
}
