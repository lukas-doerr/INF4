import java.util.ArrayList;
import java.util.List;
import java.io.File;  // Import the File class
import java.io.FileNotFoundException;  // Import this class to handle errors
import java.util.Scanner; // Import the Scanner class to read text files

public class Main {
    public static void main(String[] args) {
        BTree bst = new BTree(3);

        try {
            File myObj = new File("../../Files/seq0.txt");
            Scanner myReader = new Scanner(myObj);
            int i = 10;
            while (myReader.hasNextLine()) {
            	
                String data = myReader.nextLine();
                int val = Integer.parseInt(data);
                System.out.println(val);
		
                bst.insert(val);
                //bst.walk(bst.root);
                i--;
            }
            myReader.close();
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }

        // Print the tree height
        // int height = bst.height();
        // System.out.println("Tree height: " + height);

        // Print the keys in the tree using inorder traversal
        bst.walk();

        // Search for a key
        int key = 58;
        /*Node result = bst.search(bst.root, key);
        if (result != null) {
            System.out.println("Key " + key + " found in the tree!");
        } else {
            System.out.println("Key " + key + " not found in the tree.");
        }*/
    }
}


class BTree {
    private Node root;
    private int t; // minimum degree

    private class Node {
        private List<Integer> keys;
        private List<Node> children;
        private boolean leaf;

        Node(boolean leaf) {
            this.keys = new ArrayList<>();
            this.children = new ArrayList<>();
            this.leaf = leaf;
        }
    }

    BTree(int t) {
        this.root = new Node(true);
        this.t = t;
    }

    public void insert(int key) {
        Node r = this.root;
        if (r.keys.size() == (2 * t - 1)) {
            Node s = new Node(false);
            this.root = s;
            s.children.add(r);
            splitChild(s, 0);
            insertNonFull(s, key);
        } else {
            insertNonFull(r, key);
        }
    }

    private void insertNonFull(Node x, int key) {
        int i = x.keys.size() - 1;
        if (x.leaf) {
            while (i >= 0 && key < x.keys.get(i)) {
                i--;
            }
            x.keys.add(i + 1, key);
        } else {
            while (i >= 0 && key < x.keys.get(i)) {
                i--;
            }
            i++;
            if (x.children.get(i).keys.size() == (2 * t - 1)) {
                splitChild(x, i);
                if (key > x.keys.get(i)) {
                    i++;
                }
            }
            insertNonFull(x.children.get(i), key);
        }
    }

    private void splitChild(Node x, int i) {
        Node y = x.children.get(i);
        Node z = new Node(y.leaf);
        x.keys.add(i, y.keys.get(t - 1));
        x.children.add(i + 1, z);

        for (int j = 0; j < t - 1; j++) {
            z.keys.add(y.keys.get(j + t));
        }
        if (!y.leaf) {
            for (int j = 0; j < t; j++) {
                z.children.add(y.children.get(j + t));
            }
        }

        for (int j = t - 1; j >= 0; j--) {
            y.keys.remove(j + t - 1);
        }
        if (!y.leaf) {
            for (int j = t; j >= 0; j--) {
                y.children.remove(j + t);
            }
        }
    }

    public void walk() {
        traverse(root);
    }

    private void traverse(Node node) {
        if (node != null) {
            int i;
            for (i = 0; i < node.keys.size(); i++) {
                if (!node.leaf) {
                    traverse(node.children.get(i));
                }
                System.out.print(node.keys.get(i) + " ");
            }

            if (!node.leaf) {
                traverse(node.children.get(i));
            }
        }
    }
}

