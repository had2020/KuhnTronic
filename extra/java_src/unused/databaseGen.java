package unused;
import java.sql.*;

public class databaseGen {
    public static void main(String[] args) {
        // It's a good code base 👌🏻
        Connection c = null;

        try (Connection con = DriverManager.getConnection("jdbc:sqlite:kuhn_training_data.db"); ) {
            Statement st = con.createStatement();

            String sql = """
            CREATE TABLE IF NOT EXISTS dataset (
                id INTEGER PRIMARY KEY,
                input0 DOUBLE NOT NULL,
                input1 DOUBLE NOT NULL,
                input2 DOUBLE NOT NULL,
                input3 DOUBLE NOT NULL,
                input4 DOUBLE NOT NULL,
                input5 DOUBLE NOT NULL,
                output DOUBLE NOT NULL,
            )
            """;



            st.execute(sql);




        } catch (SQLException ex) {
            System.out.println("SQL Exception! " + ex);
        }
    }
}
