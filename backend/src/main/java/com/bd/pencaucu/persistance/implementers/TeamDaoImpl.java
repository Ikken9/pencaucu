package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.mappers.models.TeamMapper;
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
        String sql = "SELECT name, group_letter, flag_image FROM Teams WHERE name = ?";
        List<Team> teams = jdbcTemplate.query(sql, new TeamMapper(), id);

        if (!teams.isEmpty()) {
            return teams.get(0);
        }

        return null;
    }

    @Override
    public List<Team> findAll() {
        String sql = "SELECT name, group_letter, flag_image FROM Teams";

        return jdbcTemplate.query(sql, new TeamMapper());
    }

    @Override
    public List<String> findAllTeamNames() {
        String sql = "SELECT t.name FROM Teams t";

        return jdbcTemplate.query(sql, new TeamNameMapper());
    }

    @Override
    public void save(Team team) {
        String sql = "INSERT INTO Teams (name, group_letter, flag_image) VALUES (?, ?, ?)";

        jdbcTemplate.update(sql,
                team.getName(),
                team.getGroupLetter(),
                team.getPictureUrl());
    }

    @Override
    public void update(Team team) {
        String sql = "UPDATE Teams SET group_letter = ?, flag_image = ? WHERE name = ?";

        jdbcTemplate.update(sql, team.getGroupLetter(), team.getPictureUrl(), team.getName());
    }

    @Override
    public void delete(String id) {
        String sql = "DELETE FROM Teams WHERE name = ?";
        jdbcTemplate.update(sql, id);
    }
}
