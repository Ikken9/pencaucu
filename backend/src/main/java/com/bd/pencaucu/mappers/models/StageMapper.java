package com.bd.pencaucu.mappers.models;

import com.bd.pencaucu.models.Stage;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class StageMapper implements RowMapper<Stage> {
    @Override
    public Stage mapRow(ResultSet rs, int rowNum) throws SQLException {
        Stage stage = new Stage();
        stage.setId(rs.getInt("id"));
        stage.setName(rs.getString("name"));

        return stage;
    }
}
