public class PlaceCone extends Action {
    JunctionItem itemToPlace;
    Junction juncToPlaceOn;
    public PlaceCone(double timeItTakes, JunctionItem item, Junction junc) {
        super(timeItTakes);
        itemToPlace = item;
        juncToPlaceOn = junc;
    }
    @Override
    public void doAction(Robot r) {
        juncToPlaceOn.addItem(itemToPlace);
    }
}
