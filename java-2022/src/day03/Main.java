package day03;

import day02.Hand;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * Day 03 - Rucksack Reorganization
 */
public class Main {


    static int priority(Character c) {
        if (c >= 'a' && c <= 'z') {
            return (int) c - 97 + 1;
        } else if (c >= 'A' && c <= 'Z') {
            return (int) c - 65 + 27;
        } else {
            throw new RuntimeException("charcter is not in alphabetic ASCII range: " + c);
        }
    }

    /**
     * fid the common character that occurs in both the left half and right half of the input string, 's'
     * and return it
     */
    static Character[] findCommon(String s1, String s2) {
        HashSet<Character> common = new HashSet<>();

        HashSet<Character> leftSet = s1.chars().mapToObj(e -> (char) e).collect(Collectors.toCollection(HashSet::new));
        for (char c : s2.toCharArray()) {
            if (leftSet.contains(c)) {
                common.add(c);
            }
        }

        Character [] carr = new Character[common.size()];
        common.toArray(carr);

        return carr;
    }

    static void part1() {

        File f = new File("./input-2022/d03-input.txt");
        int prioritySum = 0;

        try(Scanner scanner = new Scanner(f)) {
            while (scanner.hasNextLine()) {
                String line = scanner.nextLine();
                // find the common character in each side of the line
                Character[] common = Main.findCommon(
                        line.substring(0, line.length() / 2),
                        line.substring(line.length() / 2)
                );

                // find the priority value of common character(s) and add them to the sum
                for (Character c : common) {
                    prioritySum += Main.priority(c);
                }
            }
            System.out.println("Total Priority sum of common chars is " + prioritySum);

        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        }
    }

    static void part2() {
        int prioritySum = 0;
        List<String> lines;

        try(Stream<String> l = Files.lines(Path.of("./input-2022/d03-input.txt"))) {
            // collect all the lines of the file into a list of lines
            lines = l.collect(Collectors.toList());
            for (int i = 0; i < lines.size(); i += 3) {
                String line1 = lines.get(i);
                String line2 = lines.get(i+1);
                String line3 = lines.get(i+2);

                Character[] first2Common = Main.findCommon(line1, line2);
                String commonStr = Arrays.toString(first2Common);
                Character[] all3Common = Main.findCommon(commonStr, line3);

                // find the priority value of common character(s) and add them to the sum
                for (Character c : all3Common) {
                    prioritySum += Main.priority(c);
                }
            }
            System.out.println("Part 2 Priority sum of common chars is " + prioritySum);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public static void main(String[] args) {
        Main.part2();
    }
}
