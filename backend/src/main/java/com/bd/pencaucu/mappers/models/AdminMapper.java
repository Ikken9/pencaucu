package com.bd.pencaucu.mappers.models;

import com.bd.pencaucu.domain.models.Admin;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class AdminMapper implements RowMapper<Admin> {

    @Override
    public Admin mapRow(ResultSet rs, int rowNum) throws SQLException {
        Admin admin = new Admin();
        admin.setEmail(rs.getString("admin_email"));

        return admin;
    }
}
