package day06;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashSet;

public class TuningTrouble {

    // dupeSet is used to test for duplicate characters in a slice of a string
    private final HashSet<Character> dupeSet = new HashSet<>(14);

    boolean hasDuplicates(String s) {
        this.dupeSet.clear();
        for(Character c : s.toCharArray()) {
            if (!dupeSet.add(c)) {
                return true;
            }
        }
        return false;
    }

    String readInput() throws IOException {
        Path inputPath = Path.of("./input-2022/d06-input.txt");
        return Files.readString(inputPath);
    }

    // find the start of the packet
    public void part1() {
        try {
            int start = 0;
            int end = 4;
            String sub = "";
            String input = this.readInput();

            while (end < input.length()) {
                sub = input.substring(start, end);
                if (!hasDuplicates(sub)) {
                    break;
                }
                start++;
                end++;
            }
            System.out.printf("start of packet is %s at %d\n", sub, end);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public void part2() {
        try {
            int start = 0;
            int end = 14;
            String sub = "";
            String input = this.readInput();

            while (end < input.length()) {
                sub = input.substring(start, end);
                if (!hasDuplicates(sub)) {
                    break;
                }
                start++;
                end++;
            }
            System.out.printf("start of message is %s at %d\n", sub, end);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }




    public static void main(String[] args) {
        TuningTrouble tt = new TuningTrouble();
        tt.part1();
        tt.part2();
    }
}
