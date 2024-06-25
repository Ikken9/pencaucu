package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.Team;

import java.util.List;

public interface TeamDao {
    Team findById(String id);
    List<Team> findAll();
    List<String> findAllTeamNames();
    void save(Team team);
    void update(Team team);
    void delete(String id);
}
