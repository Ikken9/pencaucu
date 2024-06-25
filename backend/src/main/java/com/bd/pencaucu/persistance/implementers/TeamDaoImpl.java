package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.mappers.models.TeamNameMapper;
import com.bd.pencaucu.models.Team;
import com.bd.pencaucu.persistance.interfaces.TeamDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@RequiredArgsConstructor
public class TeamDaoImpl implements TeamDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public Team findById(String id) {
        return null;
    }

    @Override
    public List<Team> findAll() {
        return List.of();
    }

    @Override
    public List<String> findAllTeamNames() {
        String sql = "SELECT t.name FROM Teams t";

        return jdbcTemplate.query(sql, new TeamNameMapper());
    }

    @Override
    public void save(Team team) {

    }

    @Override
    public void update(Team team) {

    }

    @Override
    public void delete(String id) {

    }
}
