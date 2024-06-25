package com.bd.pencaucu.mappers.models.dto;

import com.bd.pencaucu.models.dto.RankDTO;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class RankDTOMapper implements RowMapper<RankDTO> {
    @Override
    public RankDTO mapRow(ResultSet rs, int rowNum) throws SQLException {
        RankDTO rank = new RankDTO();
        rank.setTeamName(rs.getString("team_name"));
        rank.setFinalPosition(rs.getInt("final_position"));

        return rank;
    }
}
