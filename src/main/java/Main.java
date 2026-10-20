public final class Main {
    private Main() {}

    public static void main(String[] args) {
        int status = 0;

        // QUALITY: Comparing a value with itself is redundant.
        if (status == status) {
            System.out.println("status is unchanged");
        }

        // QUALITY: This branch is unreachable.
        if (false) {
            System.out.println("unreachable");
        }
    }
}
