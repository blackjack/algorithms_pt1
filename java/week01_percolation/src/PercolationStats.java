import edu.princeton.cs.algs4.StdIn;
import edu.princeton.cs.algs4.StdOut;
import edu.princeton.cs.algs4.StdRandom;
import edu.princeton.cs.algs4.StdStats;

public class PercolationStats {

    private double[] samples;

    public PercolationStats(int n, int trials) {

        double total = n * n;

        samples = new double[trials];

        for (int i = 0; i < trials; ++i) {
            double num = percolate(n);
            samples[i] = total / num;
        }

    }

    private static int percolate(int num) {
        Percolation p = new Percolation(num);
        do {
            int row = StdRandom.uniform(1, num + 1);
            int column = StdRandom.uniform(1, num + 1);
            p.open(row, column);
        } while (!p.percolates());
        return p.numberOfOpenSites();
    }

    public double mean() {
        return StdStats.mean(samples);
    }

    public double stddev() {
        return StdStats.stddev(samples);
    }

    public double confidenceLo() {
        double st = samples.length * samples.length;
        return mean() - (1.96 * stddev() / st);

    }

    public double confidenceHi() {
        double st = samples.length * samples.length;
        return mean() + (1.96 * stddev() / st);
    }

    public static void main(String[] args) {
        int n = StdIn.readInt();
        int trials = StdIn.readInt();
        PercolationStats stats = new PercolationStats(n, trials);

        StdOut.print("mean                    = ");
        StdOut.println(stats.mean());

        StdOut.print("stddev                  = ");
        StdOut.println(stats.stddev());

        StdOut.print("95% confidence interval = [");
        StdOut.print(stats.confidenceLo());
        StdOut.print(", ");
        StdOut.print(stats.confidenceHi());
        StdOut.println("]");
    }
}