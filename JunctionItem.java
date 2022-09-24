public enum JunctionItem {
    TeamOneCone(Robot.Team.One),
    TeamTwoCone(Robot.Team.Two),
    TeamOneBeacon(Robot.Team.One),
    TeamTwoBeacon(Robot.Team.Two);
    public final Robot.Team team;
    JunctionItem(Robot.Team team) {
        this.team = team;
    }

}
