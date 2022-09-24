import org.jetbrains.annotations.NotNull;

import java.util.Arrays;
import java.util.Comparator;
import java.util.Optional;
import java.util.stream.Stream;

public class Simulation {
    static double gridSquareSize = 1.0;
    static double timeStep = 1.0;
    // Mapped out from the diagram in the manual, 0,0 is left-hand corner.
    static Junction[] junctions;
    static Robot robotOne;
    static Robot robotTwo;
    static float time;
    static int numRobotOneCones;
    static int numRobotTwoCones;

    static Strategy moveToZero = (robot) -> {
        robot.goalPos = new double[]{0.0, 0.0};
        robot.goalAngle = 0.0;
        return null;
    };
    static Strategy moveToClosestCone = (robot) -> {
        Action placeCone = tryRobotPlaceCone(robot);
        if (placeCone != null) {
            return placeCone;
        };
        Action pickUpCone = tryGiveRobotCone(robot);
        if (pickUpCone != null) {
            return pickUpCone;
        }
        if (robot.itemHeld == null) {
            robot.goalPos = robot.subStationPos;
            return null;
        }
        else {
            Stream<Junction> targets = Arrays.stream(junctions).filter(junction -> junction.getTop().team != robot.team);
            // Find minimum distance to the robot.
            Junction target = targets.min(Comparator.comparing(Junction::getPos, (p1, p2) -> {
                return Double.compare(Math.hypot(p1[0] - robot.getPos()[0], p1[1] - robot.goalPos[1]),
                        Math.hypot(p1[0] - robot.getPos()[0], p1[1] - robot.goalPos[1]));
            })).orElse(null);
            robot.goalPos = target.getPos();
            return null;
        }
    };
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
        numRobotOneCones = 30;
        numRobotTwoCones = 30;
    }
    private static void genRobotOne() {
        double accelRate = 0.0;
        double velCap = 0.5;
        // team one is blue
        double[] pos = new double[] {0.0, 3.0 * gridSquareSize}; // This isn't technically correct.
        double[] subPos = new double[] {0.0, 3.0 * gridSquareSize};
        double angAccelRate = 0.0;
        double angVelCap = Math.PI/2;
        double angle = 0.0; // should be facing straight right
        Strategy robotOneStrat = moveToZero;
        robotOne = new Robot(accelRate, velCap, pos, angAccelRate, angVelCap, angle, subPos, robotOneStrat);
    }
    private static void genRobotTwo() {
        double accelRate = 0.0;
        double velCap = 0.5;
        // team one is blue
        double[] pos = {6.0 * gridSquareSize, 3.0 * gridSquareSize}; // This isn't technically correct.
        double[] subPos = {6.0 * gridSquareSize, 3.0 * gridSquareSize};
        double angAccelRate = 0.0;
        double angVelCap = Math.PI/2;
        double angle = Math.PI; // should be facing straight left
        Strategy robotTwoStrat = moveToZero;
        robotTwo = new Robot(accelRate, velCap, pos, angAccelRate, angVelCap, angle, subPos, robotTwoStrat);
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
    public static Action tryGiveRobotCone(Robot r) {
        if (r.getPos() != r.getSubStationPos()) {
            return false;
        }
        if (r.getTeam() == Robot.Team.One) {
            if (numRobotOneCones == 0) {
                return false;
            }
            numRobotOneCones -= 1;
            r.itemHeld = JunctionItem.TeamOneCone;
        }
        else {
            if (numRobotTwoCones == 0) {
                return false;
            }
        }
        return true;
    }
    public static Action tryRobotPlaceCone(Robot r) {
        Stream<Junction> js = Arrays.stream(junctions);
        Optional<Junction> j = js.filter(junc -> junc.getPos() == r.getPos()).findFirst(); // Should only give back one
        if (j.isPresent() && r.getItemHeld() != null) {
            j.get().addItem(r.getItemHeld());
            r.setItemHeld(null);
            return true;
        }
        else {
            return false;
        }
    }
}
