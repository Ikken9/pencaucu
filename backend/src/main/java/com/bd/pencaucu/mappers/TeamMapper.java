package com.bd.pencaucu.mappers;

import java.sql.Blob;
import java.sql.ResultSet;
import java.sql.SQLException;

import org.springframework.jdbc.core.RowMapper;

import com.bd.pencaucu.domain.models.Team;

public class TeamMapper implements RowMapper<Team>{
    @Override
    public Team mapRow(ResultSet rs, int rowNum) throws SQLException {
        Team team = new Team();
        String teamName = rs.getString("name");
        Blob teamPicture = rs.getBlob("flag");
        
        team.setName(teamName);
        team.setPicture(teamPicture);

        return team;
    }
}
