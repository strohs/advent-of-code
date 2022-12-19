package day02;

public enum Hand {
    ROCK(1),
    PAPER(2),
    SCISSORS(3);

    private final int score;
    Hand(int score) {
        this.score = score;
    }

    public static Hand fromString(String s) {
        return switch (s) {
            case "A", "X" -> Hand.ROCK;
            case "B", "Y" -> Hand.PAPER;
            case "C", "Z" -> Hand.SCISSORS;
            default -> throw new RuntimeException("invalid string for Hand: " + s);
        };
    }
}
