package day07;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class NoSpaceLeft {

    Pattern CD_RE = Pattern.compile("\\$ cd (.+)");
    Pattern LS_RE = Pattern.compile("\\$ ls.*");
    Pattern DIR_RE = Pattern.compile("dir (.+)");
    Pattern FILE_RE = Pattern.compile("(\\d+) (.+)");

    /**
     * parse the input data into a tree. The tree is a kind of b-tree where each node can have one or more child nodes
     *
     * @return the root node of the tree
     */
    Node parseToTree(List<String> data) {

        Node root = new Node(null, 0, "/", true, new ArrayList<>());
        Node curr = root;

        for (String line : data) {
            Matcher cd_matcher = CD_RE.matcher(line);
            Matcher ls_matcher = LS_RE.matcher(line);
            Matcher dir_matcher = DIR_RE.matcher(line);
            Matcher file_matcher = FILE_RE.matcher(line);


            if (ls_matcher.matches()) {
                // do nothing
            } else if (cd_matcher.matches()) {
                String dirName = cd_matcher.group(1);
                switch (dirName) {
                    case ".." -> curr = curr.parent;
                    case "/" -> {
                        // do nothing, this is the first line of input
                    }
                    default -> // find child dir node that matches name
                            curr = curr.findChildDir(dirName);
                }
            } else if (dir_matcher.matches()) {
                // create a new dir node and add it as a child of the current node
                Node dir = Node.newDirNode(dir_matcher.group(1));
                curr.addChild(dir);
            } else if (file_matcher.matches()) {
                Node file = Node.newFileNode(file_matcher.group(2), Long.parseLong(file_matcher.group(1)));
                curr.addChild(file);
            } else {
                throw new RuntimeException("could not match input line:" + line);
            }
        }
        return root;
    }

    List<String> readInputFile(String path) {
        try {
            File f = new File(path);
            Scanner scanner = new Scanner(f);
            List<String> lines = new ArrayList<>();
            while (scanner.hasNextLine()) {
                String line = scanner.nextLine();
                lines.add(line);
            }
            return lines;
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        }
    }

    /**
     * computes the sizes of each node using a PostOrder traversal of the tree stored in 'root'
     *
     * @param root Node of the tree to traverse
     */
    void computeSizes(Node root) {
        Stack<Node> s1, s2;
        s1 = new Stack<>();
        s2 = new Stack<>();

        s1.push(root);

        while (!s1.isEmpty()) {
            Node temp = s1.pop();
            s2.push(temp);
            // push all child directories onto s1
            List<Node> childDirs = temp.getChildDirs();
            s1.addAll(childDirs);
        }

        while (!s2.isEmpty()) {
            Node temp = s2.pop();
            temp.computeSize();
            //System.out.println(temp);
        }
    }

    // find all directories with a total size of at MOST 100_000.
    // What is the sum of the total sizes of those directories? (answer: 1582412)
    static void part1() {
        String inputPath = "./input-2022/d07-input.txt";
        NoSpaceLeft nsl = new NoSpaceLeft();
        var lines = nsl.readInputFile(inputPath);

        Node root = nsl.parseToTree(lines);
        nsl.computeSizes(root);

        // walk the nodes and filter the Dirs that are <= 100_000
        List<Node> filtered = new ArrayList<>();
        Stack<Node> nodes = new Stack<>();
        nodes.push(root);
        while (!nodes.isEmpty()) {
            Node n = nodes.pop();
            if (n.isDir && n.size <= 100_000) {
                filtered.add(n);
                System.out.println("found node " + n);
            }
            nodes.addAll(n.getChildDirs());
        }
        long sum = filtered.stream().mapToLong(n -> n.size).sum();
        System.out.println("total sum of dirs " + sum);
    }

    // find the smallest directory that if deleted would free up enough space to run the update
    // answer is dir 'hzzs' with size: 3696336
    static void part2() {
        long total_space = 70_000_000;
        long needed_free_space = 30_000_000;

        String inputPath = "./input-2022/d07-input.txt";
        NoSpaceLeft nsl = new NoSpaceLeft();
        var lines = nsl.readInputFile(inputPath);

        Node root = nsl.parseToTree(lines);
        nsl.computeSizes(root);

        long curr_free_space = total_space - root.size;
        long target_space = needed_free_space - curr_free_space; // 3_598_596
        // find the directory closes to target_space
        // walk the nodes and filter the Dirs that are <= 100_000
        PriorityQueue<Node> minHeap = new PriorityQueue<>(Comparator.comparingLong(node -> node.size));
        Stack<Node> nodes = new Stack<>();
        nodes.push(root);
        while (!nodes.isEmpty()) {
            Node n = nodes.pop();
            if (n.isDir && n.size >= target_space) {
                minHeap.add(n);
                System.out.println("found node " + n);
            }
            nodes.addAll(n.getChildDirs());
        }
        Node smallest_dir = minHeap.remove();
        System.out.println("smallest node <= " + target_space + "= " + smallest_dir);
    }

    public static void main(String[] args) {
        NoSpaceLeft.part1();
        NoSpaceLeft.part2();
    }
}
