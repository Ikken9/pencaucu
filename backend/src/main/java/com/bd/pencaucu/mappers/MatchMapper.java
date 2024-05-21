package com.bd.pencaucu.mappers;

import com.bd.pencaucu.domain.models.Match;
import com.bd.pencaucu.domain.models.Team;
import org.springframework.jdbc.core.RowMapper;

import java.sql.Blob;
import java.sql.ResultSet;
import java.sql.SQLException;

public class MatchMapper implements RowMapper<Match> {

    @Override
    public Match mapRow(ResultSet rs, int rowNum) throws SQLException {
        Match match = new Match();
        String firstTeamName = rs.getString("first_team_name");
        String secondTeamName = rs.getString("second_team_name");
        Blob firstTeamPicture = rs.getBlob("first_team_flag");
        Blob secondTeamPicture = rs.getBlob("second_team_flag");

        Team team1 = new Team();
        team1.setName(firstTeamName);
        team1.setPicture(firstTeamPicture);

        Team team2 = new Team();
        team2.setName(secondTeamName);
        team2.setPicture(secondTeamPicture);

        match.setFirstTeam(team1);
        match.setSecondTeam(team2);
        match.setFirstTeamScore(rs.getInt("first_team_score"));
        match.setSecondTeamScore(rs.getInt("second_team_score"));
        match.setDate(rs.getDate("date"));

        return match;
    }
}
