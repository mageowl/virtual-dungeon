public class Test {
  public static void main(String[] args) throws InterruptedException {
    Robot robot = new Robot();
    robot.move(Robot.Dir.UP);
    while (true) {
      Thread.sleep(100);
    }
  }
}
