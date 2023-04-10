package day10;

public record Addx(int amount) implements Command {

    public static int cycleTime = 2;

    @Override
    public int getAmount() {
        return this.amount;
    }

    @Override
    public int cycleTime() {
        return cycleTime;
    }
}
