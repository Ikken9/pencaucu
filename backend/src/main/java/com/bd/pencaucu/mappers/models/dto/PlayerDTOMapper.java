package com.bd.pencaucu.mappers.models.dto;

import com.bd.pencaucu.models.dto.PlayerDTO;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class PlayerDTOMapper implements RowMapper<PlayerDTO> {

    @Override
    public PlayerDTO mapRow(ResultSet rs, int rowNum) throws SQLException {
        PlayerDTO playerDTO = new PlayerDTO();
        playerDTO.setUsername(rs.getString("username"));
        playerDTO.setPoints(rs.getInt("player_score"));
        playerDTO.setProfilePictureUrl(rs.getString("profile_picture"));

        return playerDTO;
    }
}
