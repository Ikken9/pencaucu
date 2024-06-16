package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.models.Team;
import com.bd.pencaucu.persistance.interfaces.TeamDao;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public class TeamDaoImpl implements TeamDao {
    @Override
    public Team findById(String id) {
        return null;
    }

    @Override
    public List<Team> findAll() {
        return List.of();
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
