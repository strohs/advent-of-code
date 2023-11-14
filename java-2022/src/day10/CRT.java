package day10;

import java.util.Arrays;

/**
 * Representation of the CRT
 */
public class CRT {

    private int width = 40;
    private int height = 6;

    // char used to indicate a lit pixel on the CRT
    private final char LIT = '#';
    private final char DARK = '.';
    private char[] crt;

    public CRT(int width, int height) {
        this.width = width;
        this.height = height;
        this.crt = new char[this.width * this.height];
        Arrays.fill(this.crt, this.DARK);
    }

    // checks of pixels should be lit based on the passed in center_point index value
    // center_point is assumed to be within the bounds of the crt
    public void check_draw(int cycle, int center_point, int width) {
        // compute start/end bounds of the sprite based on the center_point and width of the sprite
        int scanline_pos = cycle % this.width;
        int start = center_point - (width / 2);
        int end = center_point + (width / 2);
        if (start < 0) start = (this.crt.length) + start;
        if (end >= this.crt.length) end = end % this.crt.length;

        if (scanline_pos == start || scanline_pos == center_point || scanline_pos == end) {
            // compute 1D draw index
            int row = cycle / this.width;
            int index = row * this.width + scanline_pos;
            System.out.println("Lit: " + row + ", " + scanline_pos + "   1dIndex=" + index);
            this.crt[index] = this.LIT;
        }
    }

    //convert a one dimensional index into a two dimensional row/col index
    private int[] to_row_col(int index) {
        if (index < 0 || index > this.width * this.height) throw new RuntimeException("index is out of bounds " + index);
        int row = index / this.height;
        int col = index % this.width;
        return new int[] {row, col};
    }


    public void print_crt() {
        for (int r = 0; r < this.height; r++) {
            for (int c = 0; c < this.width; c++) {
                int index = r * this.width + c;
                System.out.print(this.crt[index]);
            }
            System.out.println();
        }
    }
}
