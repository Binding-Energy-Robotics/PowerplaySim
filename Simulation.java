import org.jetbrains.annotations.NotNull;

public class Simulation {
    static double gridSquareSize = 1.0;
    static double timeStep = 1.0;
    // Mapped out from the diagram in the manual, 0,0 is left-hand corner.
    static Junction[] junctions;
    static Robot robotOne;
    static Robot robotTwo;
    static float time;
    private static void fillInGameState() {
        // I just think this should be a function so that way we can collapse it later.
        junctions = new Junction[25];
        for (double i = 1; i < 6; i++) {
            for (double j = 1; j < 6; j++) {
                Junction.Level level;
                double[] pos = new double[] {i * gridSquareSize, j * gridSquareSize};
                if((i % 2 == 1) && (j % 2 == 1)) {
                    level = Junction.Level.Ground;
                }
                else if ((i == 1 || i == 5) && (j == 1 || j == 5)) {
                    level = Junction.Level.Low;
                }
                else if((i % 2 == 0) && (j % 2 == 0)) {
                    level = Junction.Level.Middle;
                }
                else {
                    level = Junction.Level.High;
                }
                junctions[(int)(i-1) * 5 + (int)j - 1] = new Junction(pos, level);
            }
        }

    }
    private static void genRobotOne() {
        double accelRate = 0.0;
        double velCap = 0.5;
        // team one is blue
        double[] pos = new double[] {0.0, 3.0 * gridSquareSize};
        double angAccelRate = 0.0;
        double angVelCap = Math.PI/2;
        double angle = 0.0; // should be facing straight right
        Strategy robotOneStrat = (robot) -> {
            robot.goalPos = new double[]{0.0, 0.0};
            robot.goalAngle = 0.0;
            return null;
        };
        robotOne = new Robot(accelRate, velCap, pos, angAccelRate, angVelCap, angle, robotOneStrat);
    }
    private static void genRobotTwo() {
        double accelRate = 0.0;
        double velCap = 0.5;
        // team one is blue
        double[] pos = {6.0 * gridSquareSize, 3.0 * gridSquareSize};
        double angAccelRate = 0.0;
        double angVelCap = Math.PI/2;
        double angle = Math.PI; // should be facing straight left
        Strategy robotTwoStrat = (robot) -> {
            robot.goalPos = new double[]{0.0, 0.0};
            robot.goalAngle = 0.0;
            return null;
        };
        robotTwo = new Robot(accelRate, velCap, pos, angAccelRate, angVelCap, angle, robotTwoStrat);
    }
    public static void main(String[] args) {
        genRobotOne();
        genRobotTwo();
        fillInGameState();
        run();
    }
    private static void run() {
        while (time < 150) {
            step();
            printDebug();
            time += timeStep;
        }
    }
    private static void printDebug() {
        System.out.println("Junctions: ");
        for (Junction j : junctions) {
            System.out.println(j);
        }
        System.out.printf("Current Time: %f%n", time);
        System.out.print("Robot one: ");
        System.out.println(robotOne);
        System.out.print("Robot two: ");
        System.out.println(robotTwo);
    }
    private static void step() {
        stepRobot(robotOne);
        stepRobot(robotTwo);
    }
    private static void stepRobot(@NotNull Robot r) {
        // first, see if we can make a move
        if (r.getCurrentAction() == null) {
            r.setCurrentAction(r.decideAction());
        }
        // If it's still null, it means that the robot wants to move
        if (r.getCurrentAction() == null) {
            r.move();
        }
        else {
            boolean can = r.getCurrentAction().updateTimeLeft(timeStep);
            if (can) {
                r.getCurrentAction().doAction(r);
            }
        }
    }
    public static float getTime() {
        return time;
    }
    public static double getTimeStep() {
        return timeStep;
    }

    public static boolean isEndgame() {
        return (time > 120);
    }
}
