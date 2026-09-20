import java.awt.*;
import java.util.Random;

//robot class
class MyRobot {
    static Random random = new Random();

    static int timeSinceDoor = 0;
    static Robot robot = new Robot();

    public static void main(String[] args) {
        Robot.Tile[][] scans;
        //robot.move("up")

        //robot.attack("up")
        //robot.scan(-2, 0)

        /* ROBOT WALL COINS EMPTY*/

                      while (true) {
            scans = scanning();


              for (int x = 0; x < scans.length; x++) {
                            for (int y = 0; y < scans[x].length; y++) {
                                 if (scans[x][y] == Robot.Tile.COINS) {
                        if (x - 3 < 0 && y - 3 < 0) {
                            for (int p = x - 3; p < 0; p++) {
                                robot.attack(Robot.Dir.LEFT);
                                    }
                              for (int p = y - 3; p < 0; p++) {

                                robot.attack(Robot.Dir.UP);
                            }
                        } else if (x - 3 < 0 && y - 3 > 0) {
                            for (int p = x - 3; p < 0; p++) {
                                robot.attack(Robot.Dir.LEFT);
                                  }
                            for (int p = y - 3; p > 0; p--) {


                                robot.attack(Robot.Dir.DOWN);

                            }


                            } else if (x - 3 > 0 && y - 3 > 0) {
                      
                              for (int p = x - 3; p > 0; p--) {
                                robot.move(Robot.Dir.RIGHT);
                            }
                            for (int p = y - 3; p > 0; p--) {


                                robot.attack(Robot.Dir.DOWN);
                                   }
                         } else if (x - 3 > 0 && y - 3 < 0) {
                            for (int p = x - 3; p > 0; p--) {
                                  robot.attack(Robot.Dir.RIGHT);
                            }
                                for (int p = y - 3; p > 0; p--) {
                                       robot.attack(Robot.Dir.UP);
                            }
                        }
                     } else if (scans[x][y] == Robot.Tile.ROBOT) {
                           do {
                            Point p = Efficiancy();
                            p.x = x;
                                  p.y = y;
                              if (x - 3 < 0) {
                                robot.attack(Robot.Dir.LEFT);
                                   } else {
                                 robot.attack(Robot.Dir.RIGHT);
                            }
                              if (y - 3 < 0) {
                                      robot.attack(Robot.Dir.UP);
                            } else {
                                robot.attack(Robot.Dir.DOWN);
                            }
                            // attackStuff(x, y);
                            } while (scanning2());
                    }
                }
            }
            //movestuff heaeeeaafasd
        }
    }

    public static Robot.Tile[][] scanning() {
                Robot.Tile[][] grid = new Robot.Tile[7][7];
        for (int x = 0; x < 7; x++) {
                   for (int y = 0; y < 7; y++) {


                if (x == 3 & y == 3) {
                    grid[x][y] = Robot.Tile.EMPTY;

                } else {
                            grid[x][y] = robot.scan(x - 3, y - 3);
                }
                }
        }
        return grid;
    }

    public static boolean scanning2() {


             for (int x = 0; x < 7; x++) {
            for (int y = 0; y < 7; y++) {
                if (x != 3 && y != 3 && robot.scan(x - 3, y - 3) == Robot.Tile.ROBOT)
                         return true;
            }
        }
        return false;
    }


      public static Point Efficiancy() {
        for (int x = 0; x < 7; x++) {
            for (int y = 0; y < 7; y++) {

                      if (robot.scan(x - 3, y - 3) == Robot.Tile.ROBOT)

                    return new Point (x - 3, y - 3);
            }
        }
             return  null;
    }
}


//if robot scans a wall
//have wall scan
// move 2 spaces in other direction
//other direction = weighted direction

//if randomNumber = 4
//move weightedDir
//if randomNumber = 3
//move up
//if randomNumber = 2
//move left
//if random number = 1
//move down
//if random number = 0
//move right




