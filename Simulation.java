public class Simulation {
    static double gridSquareSize = 1.0;
    static double timeStep = 1.0;
    static boolean isEndgame = false;
    // Mapped out from the diagram in the manual, 0,0 is left-hand corner.
    static Junction[] junctions;
    static Robot robotOne;
    static Robot robotTwo;
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
                junctions[(int)i * 5 + (int)j] = new Junction(pos, level);
            }
        }

    }
    private static void genRobotOne() {
        double accelRate = 0.0;
        double velCap = 0.0;
        // team one is blue
        double[] pos = new double[] {0.0, 3.0 * gridSquareSize};
        double angAccelRate = 0.0;
        double angVelCap = 0.0;
        double angle = 0.0; // should be facing straight right
        Strategy robotOneStrat = (robot) -> {
            return null;
        };
        robotOne = new Robot(accelRate, velCap, pos, angAccelRate, angVelCap, angle, robotOneStrat);
    }
    private static void genRobotTwo() {
        double accelRate = 0.0;
        double velCap = 0.0;
        // team one is blue
        double[] pos = new double[] {6.0 * gridSquareSize, 3.0 * gridSquareSize};
        double angAccelRate = 0.0;
        double angVelCap = 0.0;
        double angle = Math.PI; // should be facing straight left
        Strategy robotTwoStrat = (robot) -> {
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
        for (double i = 0.0; i < 120.0; i += timeStep) {
            step();
        }
        isEndgame = true;
        for (double i = 0.0; i < 30.0; i += timeStep) {
            step();
        }
    }
    private static void step() {
        stepRobot(robotOne);
        stepRobot(robotTwo);
    }
    private static void stepRobot(Robot r) {
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

}
