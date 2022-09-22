import java.lang.Math;


public class Robot {
    public float accel;
    public float angaccel;
    public float velocity;
    public float angvel;
    public float velcap;
    public float angvelcap;
    public float angle;
    public float pos[];
    public Stategy strat;
    public Robot(float a, float vc, float[] position, float aa, float avc, float ang, Stategy s) {
        accel = a;
        angaccel = aa;
        velcap = vc;
        angvelcap = avc;
        pos = position;
        angle = ang;
        strat = s;

        velocity = 0;
        angvel = 0;
    }
    public void move(float[] goalpos, float goalang) {
        // For now we don't actually care about acceleration
        // this assumes that we take a second
        // First, change the angle
        if (angle < goalang) {
            if (goalang - angle < angvelcap) {
                angle = goalang;
            }
            else {
                angle += angvelcap;
            }
        }
        else {
            if (angle - goalang < angvelcap) {
                angle = goalang;
            }
            else {
                angle -= angvelcap;
            }
        }
        // Next, determine angle we need to travel at
        float dy = goalpos[1] - pos[1];
        float dx = goalpos[0] - pos[0];
        double ang = Math.atan2(dy, dx);
        double[] changes = {Math.cos(ang), Math.sin(ang)};
        pos[0] += changes[0];
        pos[1] += changes[1];

    }
    public Action decide_action(Simulation state) {
        return strat.run(this, state);
    }

}

