package com.bd.pencaucu.mappers.models;

import com.bd.pencaucu.models.Career;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class CareerMapper implements RowMapper<Career> {

    @Override
    public Career mapRow(ResultSet rs, int rowNum) throws SQLException {
        Career career = new Career();
        career.setCareerName(rs.getString("career_name"));

        return career;
    }
}
