package com.bd.pencaucu.mappers;

import com.bd.pencaucu.domain.models.Player;
import com.bd.pencaucu.domain.models.User;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class UserMapper implements RowMapper<User> {

    @Override
    public User mapRow(ResultSet rs, int rowNum) throws SQLException {
        User user = new User();
        user.setEmail(rs.getString("email"));
        user.setName(rs.getString("name"));
        user.setPassword(rs.getString("password"));
        return user;
    }
}
