package day07;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class Node {

    Node parent;

    // size of this node, which is the size of all its children nodes
    long size;

    String name;

    boolean isDir;

    List<Node> children;

    public Node(Node parent, long size, String name, boolean isDir, List<Node> children) {
        this.parent = parent;
        this.size = size;
        this.name = name;
        this.isDir = isDir;
        this.children = children;
    }

    public static Node newDirNode(String name) {
        return new Node(null, 0, name, true, null);
    }

    public static Node newFileNode(String name, long size) {
        return new Node(null, size, name, false, null);
    }

    /**
     * a node to this node's list of children
     */
    public void addChild(Node child) {
        child.parent = this;
        if (this.children == null) {
            this.children = new ArrayList<>();
        }
        this.children.add(child);
    }

    /**
     * @return the first child dir that matches the given name
     */
    public Node findChildDir(String name) {
        return this.children.stream()
                .filter(n -> n.isDir && n.name.equals(name))
                .findFirst()
                .orElseThrow(() -> new RuntimeException("could not find matching child dir " + name + " from parent dir " + this.name));
    }


    /**
     *
     * @return a List of this node's child directories
     */
    public List<Node> getChildDirs() {
        return this.children.stream()
                .filter(n -> n.isDir)
                .collect(Collectors.toList());
    }

    public void computeSize() {
        this.size = this.children.stream().mapToLong(n -> n.size).sum();
    }


    @Override
    public String toString() {
        String type = isDir ? "DIR " : "FILE";
        int cDirs = getChildDirs().size();
        long fDirs = children.stream().filter(n -> !n.isDir).count();
        String parentName = parent == null ? "null" : parent.name;

        return String.format("%s %s (%d) children:[dirs:%d files:%d]  parentDir: %s", type, name, size, cDirs, fDirs, parentName);
    }
}
