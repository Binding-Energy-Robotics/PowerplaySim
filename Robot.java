import java.lang.Math;


public class Robot {
    double accel;
    double angaccel;
    double velocity;
    double angvel;
    double velcap;
    double angvelcap;
    double angle;
    double[] pos;
    Stategy strat;
    public double getAccel() {
        return accel;
    }
    public void setAccel(double accel) {
        this.accel = accel;
    }
    public double getAngaccel() {
        return angaccel;
    }
    public void setAngaccel(double angaccel) {
        this.angaccel = angaccel;
    }
    public double getVelocity() {
        return velocity;
    }
    public void setVelocity(double velocity) {
        this.velocity = velocity;
    }
    public double getAngvel() {
        return angvel;
    }
    public void setAngvel(double angvel) {
        this.angvel = angvel;
    }
    public double getVelcap() {
        return velcap;
    }
    public void setVelcap(double velcap) {
        this.velcap = velcap;
    }
    public double getAngvelcap() {
        return angvelcap;
    }
    public void setAngvelcap(double angvelcap) {
        this.angvelcap = angvelcap;
    }
    public double getAngle() {
        return angle;
    }
    public void setAngle(double angle) {
        this.angle = angle;
    }
    public double[] getPos() {
        return pos;
    }
    public void setPos(double[] pos) {
        this.pos = pos;
    }
    public Robot(double a, double vc, double[] position, double aa, double avc, double ang, Stategy s) {
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
    public void Move(double[] goalpos, double goalang) {
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
        double dy = goalpos[1] - pos[1];
        double dx = goalpos[0] - pos[0];
        double ang = Math.atan2(dy, dx);
        double[] changes = {Math.cos(ang), Math.sin(ang)};
        pos[0] += changes[0];
        pos[1] += changes[1];

    }
    public Action DecideAction(Simulation state) {
        return strat.run(this, state);
    }

}

