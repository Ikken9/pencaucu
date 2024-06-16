package com.bd.pencaucu.mappers.models;

import com.bd.pencaucu.models.Login;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class LoginMapper implements RowMapper<Login> {

    @Override
    public Login mapRow(ResultSet rs, int i) throws SQLException {
        Login login = new Login();
        login.setEmail(rs.getString("user_email"));
        login.setPassword(rs.getString("password"));

        return login;
    }
}
