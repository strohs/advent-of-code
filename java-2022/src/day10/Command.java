package day10;

/**
 * A command that can be given to a Register
 */
public interface Command {
    int getAmount();

    /**
     * @return the cpu cycles required to complete the command
     */
    int cycleTime();

}
