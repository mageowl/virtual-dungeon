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
  }
}
