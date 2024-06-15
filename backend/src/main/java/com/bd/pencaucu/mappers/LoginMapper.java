package com.bd.pencaucu.mappers;

import com.bd.pencaucu.domain.models.Login;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class LoginMapper implements RowMapper<Login> {

    @Override
    public Login mapRow(ResultSet resultSet, int i) throws SQLException {
        Login login = new Login();
        login.setEmail(resultSet.getString("user_email"));
        login.setPassword(resultSet.getString("password"));

        return login;
    }
}
