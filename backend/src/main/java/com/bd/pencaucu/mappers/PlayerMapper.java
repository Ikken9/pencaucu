package com.bd.pencaucu.mappers;

import com.bd.pencaucu.domain.models.Player;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class PlayerMapper implements RowMapper<Player> {

    @Override
    public Player mapRow(ResultSet rs, int rowNum) throws SQLException {
        Player player = new Player();
        player.setEmail(rs.getString("email"));
        player.setCareerName(rs.getString("career_name"));

        return player;
    }
}
