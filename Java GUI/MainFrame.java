import java.awt.*;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.util.ArrayList;
import java.util.Collections;

import javax.swing.*;

public class MainFrame extends JFrame implements ActionListener {
    JButton checkButton, betButton, callButton, foldButton;
    

    MainFrame() {
        checkButton = new JButton("Check");
        betButton   = new JButton("Bet");
        callButton  = new JButton("Call");
        foldButton  = new JButton("Fold");

        styleButton(checkButton);
        styleButton(betButton);
        styleButton(callButton);
        styleButton(foldButton);


        this.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        this.setLayout(new BorderLayout());
        this.setSize(500, 500);
        this.getContentPane().setBackground(new Color(10, 50, 30)); // darker table color
        this.setVisible(true);

        JPanel panel1 = new JPanel();
        panel1.add(checkButton);
        panel1.add(betButton);
        panel1.add(callButton);
        panel1.add(foldButton);

        panel1.setBackground(new Color(10, 50, 30));
        this.add(panel1, BorderLayout.SOUTH);

         // Create an empty list to store all 52 playing cards
        ArrayList<ImageIcon> deck = new ArrayList<>();

        // Store the four card suits
        String[] suits = {"spades"};

        // Store the thirteen card ranks
        String[] ranks = {"jack", "queen", "king"};

        // Store the folder location containing the card images
        String folder =
        "KuhnTronic\\Java GUI\\PNG-cards-1.3\\";

        // Go through every suit
        for (String suit : suits) {
            // Go through every rank for the current suit
            for (String rank : ranks) {
                // Build the file path for the current card
                String path = folder + rank + "_of_" + suit + ".png";

                // Load the card image
                ImageIcon card = new ImageIcon(path);

                // Add the loaded card to the deck
                deck.add(card);
            }
        }

        // Randomly rearrange the cards in the deck
        Collections.shuffle(deck);

        // Remove and deal the first card to Player 1
        ImageIcon player1Card = deck.remove(0);

        // Remove and deal the next card to Player 2
        ImageIcon player2Card = deck.remove(0);

        // Remove the next card and keep it hidden
        ImageIcon hiddenCard = deck.remove(0);

        // Create a label to display Player 1's card
        JLabel l = new JLabel(player1Card);

        // Create a label to display Player 2's card
        JLabel m = new JLabel(player2Card);
        
         // Create a label to display second face down card
        JLabel n = new JLabel(hiddenCard);

        // create a panel
        JPanel p = new JPanel();

        // Set the deep green background
        p.setBackground(new Color(10, 50, 30)); 

        // Ensure the panel paints its background
        p.setOpaque(true);

        // Arrange the labels in one row with two columns
        p.setLayout(new GridLayout(1, 3));

        // Add the first card label to the panel
        p.add(l);

        // Add the second card label to the panel
        p.add(m);

        // Add the second card label to the panel
        p.add(n);

        // add panel to frame
        this.add(p, BorderLayout.CENTER);
    }

    void styleButton(JButton b) {
        b.setFont(new Font("Georgia", Font.BOLD, 18));
        b.setForeground(new Color(255, 215, 0));   // gold text
        b.setBackground(new Color(20, 80, 50));    // felt green
        b.setPreferredSize(new Dimension(150, 50)); 
        b.setFocusPainted(false);
        b.setOpaque(true);
        b.setBorderPainted(false);
        b.addActionListener(this);
    }

    @Override
    public void actionPerformed(ActionEvent e) {
        if (e.getSource() == checkButton) {
            System.out.println("Check");
        } else if (e.getSource() == betButton) {
            System.out.println("Bet");
        } else if (e.getSource() == callButton) {
            System.out.println("Call");
        } else if (e.getSource() == foldButton) {
          System.out.println("Fold");
        }
    }

    public static void main(String[] args) {
        new MainFrame();
    }
}