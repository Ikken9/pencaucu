package com.bd.pencaucu.mappers.models;

import java.sql.ResultSet;
import java.sql.SQLException;
import com.bd.pencaucu.models.Team;
import org.springframework.jdbc.core.RowMapper;

public class TeamMapper implements RowMapper<Team> {
    @Override
    public Team mapRow(ResultSet rs, int rowNum) throws SQLException {
        Team team = new Team();
        String teamName = rs.getString("name");
        String teamPicture = rs.getString("flag");
        
        team.setName(teamName);
        team.setPictureUrl(teamPicture);

        return team;
    }
}
