public abstract class Action {
    private double timeLeft;
    public boolean updateTimeLeft(double timeStep) {
        timeLeft -= timeStep;
        return (timeLeft < 0);
    }
    public Action(double timeItTakes) {
        timeLeft = timeItTakes;
    }
    public abstract void doAction(Robot r);
}
