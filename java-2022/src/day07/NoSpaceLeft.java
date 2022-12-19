package day07;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class NoSpaceLeft {

    Pattern CD_RE = Pattern.compile("\\$ cd (.+)");
    Pattern LS_RE = Pattern.compile("\\$ ls.*");
    Pattern DIR_RE = Pattern.compile("dir (.+)");
    Pattern FILE_RE = Pattern.compile("(\\d+) (.+)");

    /**
     * parse the input data into a tree. The tree is a kind of b-tree where each node can have one of more child nodes
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
                    case ".." -> {
                        curr = curr.parent;
                    }
                    case "/" -> {
                        // do nothing, this is the first line of input
                    }
                    default -> {
                        // find child dir node that matches name
                        Node childDir = curr.findChildDir(dirName);
                        curr = childDir;
                    }
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

    public static void main(String[] args) {
        String inputPath = "./input-2022/d07-test-input.txt";
        NoSpaceLeft main = new NoSpaceLeft();
        var lines = main.readInputFile(inputPath);
        for (String line : lines) {
            System.out.println(line);
        }
        main.parseToTree(lines);
    }
}
