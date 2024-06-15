package com.bd.pencaucu.mappers;

import com.bd.pencaucu.domain.models.Match;
import com.bd.pencaucu.dto.MatchDTO;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class MatchDTOMapper implements RowMapper<MatchDTO> {

    @Override
    public MatchDTO mapRow(ResultSet rs, int rowNum) throws SQLException {
        MatchDTO matchDTO = new MatchDTO();
        matchDTO.setId(rs.getInt("id"));
        matchDTO.setDate(rs.getDate("date"));
        matchDTO.setTeamName(rs.getString("team_name"));
        matchDTO.setFacedTeamName(rs.getString("faced_team_name"));
        matchDTO.setStadiumName(rs.getString("name"));
        matchDTO.setTeamScore(rs.getInt("team_score"));
        matchDTO.setFacedTeamScore(rs.getInt("faced_team_score"));
        matchDTO.setTeamPictureUrl(rs.getString("t1.flag_image"));
        matchDTO.setFacedTeamPictureUrl(rs.getString("t2.flag_image"));

        return matchDTO;
    }

}
