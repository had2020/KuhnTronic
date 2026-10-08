import java.util.Random;

// java Scanner learn input and output

public class initcards {
    public static void main(String[] args) {

        // gen   3 King 2 Queen 1 Joker
        // AI:
        // User:
        // Card middle: 
    Random hands = new Random();
   
    int hands1 = hands.nextInt(1,4);

    int hands2 = hands.nextInt(1,4);
    while (hands2 == hands1){
        hands2 = hands.nextInt(1,4);
    }

    int hands3 = hands.nextInt(1,4);
     while (hands3 == hands1 || hands3 == hands2){
        hands3 = hands.nextInt(1,4);
     }
     
    System.out.println(hands1);
    System.out.println(hands2);
    System.out.println(hands3);
}





}
