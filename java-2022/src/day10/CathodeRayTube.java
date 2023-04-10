package day10;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.stream.Collectors;

/**
 * Advent of Code, Day 10
 */
public class CathodeRayTube {

    Path input = Path.of("./input-2022/d10-input.txt");

    private int registerX = 1;

    Deque<Command> readInput() throws IOException {
        ArrayDeque<Command> commands = new ArrayDeque<>();
        Files.lines(this.input)
                .forEach(line -> {
                    String[] splits = line.split(" ");
                    switch (splits[0]) {
                        case "addx" -> commands.add(new Addx(Integer.parseInt(splits[1])));
                        case "noop" -> commands.add(new Noop());
                        default -> throw new IllegalArgumentException("unknown command " + splits[0]);
                    }
                });
        return commands;
    }

    /**
     * Part 1: Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles.
     * What is the sum of these six signal strengths?
     */
    public void part1() throws IOException {
        long[] testTimes = new long[] {20, 60, 100, 140, 180, 220};
        long cycle = 1;
        // signal_strength = cycle * registerX  re-computed at every 20th cycle
        // sum of signal strength
        long ssSum = 0;
        var commands = this.readInput();
        Command command = new Noop();
        long opCycle = 0;

        while (commands.size() > 0) {
            if (Arrays.binarySearch(testTimes, cycle) >= 0) {
                var signalStrength = cycle * this.registerX;
                System.out.printf("signal strength at cycle %d = %d\n", cycle, signalStrength);
                ssSum += signalStrength;
            }

            // fetch command and "begin" execution
            if (opCycle < cycle) {
                command = commands.remove();
                opCycle = command.cycleTime() + cycle;
            }

            // when opCycle == cycle, the current command has finished
            if (opCycle == cycle) {
                switch (command) {
                    case Addx ax -> {
                        this.registerX += ax.amount();
                    }
                    default -> {
                        // default is == Noop
                    }
                }
            }

            opCycle -= 1;
            cycle += 1;
        }
        System.out.printf("sum of signal strengths %d\n", ssSum);
    }

    public static void main(String[] args) throws IOException {
        CathodeRayTube crt = new CathodeRayTube();
        crt.part1();
    }
}
