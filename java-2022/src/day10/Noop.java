package day10;

public record Noop() implements Command {

    public static final int cycleTime = 1;

    @Override
    public int getAmount() {
        return 0;
    }

    @Override
    public int cycleTime() {
        return cycleTime;
    }
}
