import java.util.Scanner;

public class Robot {
  public enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
  }

  private Scanner scanner = new Scanner(System.in);
  
  public void move(Dir dir) {
    switch (dir) {
      case UP: System.out.println("\0move up"); break;
      case DOWN: System.out.println("\0move down"); break;
      case LEFT: System.out.println("\0move left"); break;
      case RIGHT: System.out.println("\0move right"); break;
    }
    scanner.nextLine();
  }
  public void attack(Dir dir) {
    switch (dir) {
      case UP: System.out.println("\0attack up"); break;
      case DOWN: System.out.println("\0attack down"); break;
      case LEFT: System.out.println("\0attack left"); break;
      case RIGHT: System.out.println("\0attack right"); break;
    }
    scanner.nextLine();
  }

  public enum Tile {
    EMPTY,
    ROBOT,
    COINS,
    WALL,
  }
  public Tile scan(int x, int y) {
    System.out.printf("\0scan %d %d\n", x, y);
    String[] res = scanner.nextLine().split(" ");
    if (!res[0].equals("tile")) {
      System.err.println("Failed to finish scan.");
    }

    switch (res[1]) {
      case "empty":
        return Tile.EMPTY;
      case "robot":
        return Tile.ROBOT;
      case "coins":
        return Tile.COINS;
      case "wall":
        return Tile.WALL;
      default:
        System.err.printf("Unrecognized tile: %s\n", res[1]);
        return Tile.EMPTY;
    }
  }
}
