import java.util.Stack;

public class Junction {
    Stack<JunctionItem> items;
    boolean capped;
    public Junction() {
        items = new Stack<>();
        capped = false;
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
            }
        return false;
    }
}
