package day02;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class FileStuff {

    public static void main(String[] args) {

        try {
            File f = new File("./input-2022/d02-input.txt");
            Scanner scanner = new Scanner(f);
            while (scanner.hasNextLine()) {
                String line = scanner.nextLine();
                String [] splits =  line.split(" ");
                Hand op = Hand.fromString(splits[0]);
                Hand pl = Hand.fromString(splits[1]);
                System.out.printf("%s : %s\n", op, pl);
            }

        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        }
    }
}
