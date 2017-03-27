import edu.princeton.cs.algs4.WeightedQuickUnionUF;

public class Percolation {

    private int x;
    private int y;
    private int last;
    private WeightedQuickUnionUF data;
    private boolean[] open;

    public Percolation(int n) {
        x = n;
        y = n;
        last = x * y + 1;
        data = new WeightedQuickUnionUF(last + 1);
        open = new boolean[last + 1];

        open[0] = true;
        open[last] = true;
    }

    private int index(int row, int column) {
        if (row == 0) {
            return 0;
        }
        if (row > y) {
            return last;
        }

        return y * (row - 1) + column;
    }

    private void checkBounds(int row, int col) {
        if (col <= 0 || col > x || row <= 0 || row > y) {
            throw new IndexOutOfBoundsException();
        }
    }

    public void open(int row, int col) {
        checkBounds(row, col);
        int idx = index(row, col);

        if (!isOpen(row, col)) {
            this.open[idx] = true;
        }


        int[][] neighbors = new int[][]{
                {row + 1, col},
                {row - 1, col},
                {row, col + 1},
                {row, col - 1}
        };


        for (int i = 0; i < neighbors.length; ++i) {
            int nrow = neighbors[i][0];
            int ncol = neighbors[i][1];

            if (ncol <= 0 || ncol > x) {
                continue;
            }

            int nidx = index(nrow, ncol);
            if (nidx == 0 || nidx == last || open[nidx]) {
                data.union(idx, nidx);
            }
        }
    }

    public boolean isOpen(int row, int col) {
        checkBounds(row, col);
        int idx = index(row, col);
        return open[idx];
    }

    public boolean isFull(int row, int col) {
        checkBounds(row, col);
        int idx = index(row, col);
        return data.connected(0, idx);
    }

    public int numberOfOpenSites() {
        int max = last;
        int count = 0;
        for (int i = 1; i < last; ++i) {
            if (open[i]) {
                ++count;
            }
        }
        return count;
    }

    public boolean percolates() {
        return data.connected(0, last);
    }
}