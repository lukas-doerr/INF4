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
        int height = bst.height();
        System.out.println("Tree height: " + height);

        // Print the keys in the tree using inorder traversal
        bst.walk(bst.root);

        // Search for a key
        int key = 58;
        Node result = bst.search(bst.root, key);
        if (result != null) {
            System.out.println("Key " + key + " found in the tree!");
        } else {
            System.out.println("Key " + key + " not found in the tree.");
        }
    }
}

class Node {
    int n;                        // Current number of keys
    List<Integer> keys;           // Keys of the node
    List<Node> childs;            // Child nodes
    boolean isLeaf;               // Indicates if it's a leaf node

    Node() {
        n = 0;
        keys = new ArrayList<>();
        childs = new ArrayList<>();
        isLeaf = true;
    }
}

interface Tree {
    Node makeNode();
    Node search(Node node, int key);
    void insert(int key);
    void splitChild(Node node, int i);
    void walk(Node node);
    void insertInNode(Node node, int key);
    int heightRoot(Node node, int h);
    int height();
}

class BTree implements Tree {
    Node root;
    int m;                           // Upper Limit of nodes

    BTree(int _m) {
        root = null;
        m = _m;
    }

    @Override
    public Node makeNode() {
        Node node = new Node();
        node.childs = new ArrayList<>();
        return node;
    }

    @Override
    public Node search(Node node, int key) {
        int i = 0;

        while (i < node.n && key > node.keys.get(i)) {
            i++;
        }

        if (i < node.n && key == node.keys.get(i)) {
            return node;
        }

        if (node.isLeaf) {
            return null;
        }

        return search(node.childs.get(i), key);
    }

    @Override
    public void insert(int key) {
        if (this.root != null) {
            Node r = this.root;
            if (r.n == (2 * this.m - 1)) {
                Node h = makeNode();
                this.root = h;
      
                h.isLeaf = false;
                h.n = 0;
                h.childs.add(r);
                System.out.println(h.childs.size());
                splitChild(h, 0);
                insertInNode(h, key);
            } else {
                insertInNode(r, key);
            }
        } else {
            Node root = makeNode();
            root.keys.add(key);
            root.n = 1;
            this.root = root;
        }
    }

@Override
public void insertInNode(Node node, int key) {
    int i = node.n - 1;

    if (node.isLeaf) {
while (i >= 0 && key < node.keys.get(i)) {
    node.keys.add(i + 1, node.keys.get(i));
    i--;
}

        node.keys.add(i + 1, key);
        node.n = node.n + 1;
    } else {
        int j = 0;
        while (j < node.n && key > node.keys.get(j)) {
            j++;
        }
        if (node.childs.get(j).n == 2 * m - 1) {
            splitChild(node, j);
            if (key > node.keys.get(j)) {
                j++;
            }
        }
        insertInNode(node.childs.get(j), key);
    }
}


    @Override
    public void walk(Node node) {
    
	if (node.isLeaf) {
	    for (int i=0; i<node.n; i++) {
		System.out.println(node.keys.get(i));
            }
        } else {
        for (int j=0; j<node.childs.size(); j++) {
        	Node child = node.childs.get(j);
        	if (child!=null) {
        	    walk(child);
        	    if (j<node.n) {
        	        System.out.println(node.keys.get(j));
        	    }
        	}
            }
        }
    }

    @Override
    public int height() {
        if (root != null) {
            return heightRoot(root, 0);
        } else {
            return 0;
        }
    }

    @Override
    public int heightRoot(Node node, int h) {
        int maxChildHeight = h;

        if (node.isLeaf) {
            return h + 1;
        }

        for (Node child : node.childs) {
            int childHeight = heightRoot(child, h + 1);
            if (childHeight > maxChildHeight) {
                maxChildHeight = childHeight;
            }
        }
        return maxChildHeight;
    }

    public void splitChild(Node x, int i) {
        Node k = x.childs.get(i);
        Node h = makeNode();
        h.isLeaf = k.isLeaf;
        h.n = m - 1;
        // System.out.println(m);
        for (int j = 0; j <= m - 2; j++) {
            h.keys.add(k.keys.get(j + m));
        }         

        if (!h.isLeaf) {
            for (int j = 0; j <= m - 1; j++) {
                h.childs.add(k.childs.get(j + m));
            }
        }

        for (int j = m; j <= k.n; j++) {
            k.childs.add(null);
        }

        for (int j = x.n - 1; j >= i+1; j--) {
            x.childs.set(j+1, x.childs.get(j));
        }
        k.n = m - 1;

        // System.out.println(x.childs.size());
        x.childs.add(h);

        for (int j = x.n - 1; j >= i+1; j--) {
            x.keys.add(j + 1, x.keys.get(j));
        }
        x.keys.add(k.keys.get(m - 1));
        x.n++;
    }
}




