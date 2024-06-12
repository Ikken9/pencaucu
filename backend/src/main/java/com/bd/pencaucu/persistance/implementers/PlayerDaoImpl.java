package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.Player;
import com.bd.pencaucu.mappers.PlayerMapper;
import com.bd.pencaucu.persistance.interfaces.PlayerDao;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public class PlayerDaoImpl implements PlayerDao {

    private final String PLAYER_NOT_FOUND_MSG = "Player with email %s not found";

    private JdbcTemplate jdbcTemplate;

    @Override
    public Player findById(String id) throws UsernameNotFoundException {
        String sql = "SELECT player_email, career FROM Players WHERE id = ?";
        List<Player> players = jdbcTemplate.query(sql, new PlayerMapper(), id);

        if (!players.isEmpty()) {
            return players.get(0);
        }

        throw new UsernameNotFoundException(String.format(PLAYER_NOT_FOUND_MSG, id));
    }

    @Override
    public List<Player> findAll() {
        String sql = "SELECT player_email, career FROM Players";

        return jdbcTemplate.query(sql, new PlayerMapper());
    }

    @Override
    public void save(Player player) {
        String sql = "INSERT INTO Players (player_email, career) VALUES (?, ?)";

        jdbcTemplate.update(sql,
                player.getEmail(),
                player.getCareer());

    }

    @Override
    public void update(Player player) {

    }

    @Override
    public void delete(String id) {
        String sql = "DELETE FROM Players WHERE id = ?";
        jdbcTemplate.update(sql, id);
    }
}
