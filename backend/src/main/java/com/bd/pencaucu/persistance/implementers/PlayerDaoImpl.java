package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.Player;
import com.bd.pencaucu.persistance.interfaces.PlayerDao;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public class PlayerDaoImpl implements PlayerDao {

    private JdbcTemplate jdbcTemplate;

    @Override
    public Player findById(String id) {
        return null;
    }

    @Override
    public List<Player> findAll() {
        return List.of();
    }

    @Override
    public void save(Player player) {

    }

    @Override
    public void update(Player player) {

    }

    @Override
    public void delete(String id) {

    }
}
