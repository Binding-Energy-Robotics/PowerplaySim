import java.util.Stack;

public class Junction {
    public enum Level {
        Ground,
        Low,
        Middle,
        High,
    }

    Stack<JunctionItem> items;
    double[] pos;
    boolean capped;

    Level level;

    public Junction(double[] p, Level l) {
        items = new Stack<>();
        capped = false;
        pos = p;
        level = l;
    }

    public JunctionItem getTop() {
        return items.peek();
    }

    public boolean addItem(JunctionItem j) {
        if (capped) {
            return false;
        }
        if ((j == JunctionItem.TeamOneBeacon) || (j == JunctionItem.TeamTwoBeacon)) {
            items = new Stack<>();
            items.push(j);
            capped = true;
        } else {
            // If junction is already owned by the same beacon
            if (items.peek() == j) {
                items.push(j);
            } else {
                items = new Stack<>();
                items.push(j);
            }
        }
        return false;
    }

    public double[] getPos() {
        return pos;
    }

    public Level getLevel() {
        return level;
    }

    public String toString() {
        String s = "Level: ";
        switch (level) {
            case Ground -> s += "Ground, ";
            case Low -> s += "Low, ";
            case Middle -> s += "Middle, ";
            case High -> s += "High, ";
        }
        if (items.size() > 0) {
            s += "Top item: " + items.peek().toString() + ", ";
        }
        else {
            s += "Top item: None, ";
        }
        s += "Capped: " + capped;
        return s;
    }
}
