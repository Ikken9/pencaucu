package com.bd.pencaucu.services;

import com.bd.pencaucu.models.Team;
import com.bd.pencaucu.persistance.interfaces.TeamDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class TeamService {
    private final TeamDao teamDao;

    public List<Team> getAllTeams() {
        return teamDao.findAll();
    }

    public List<String> getAllTeamNames() {
        return teamDao.findAllTeamNames();
    }

    public Team getTeamById(String id) {
        return teamDao.findById(id);
    }

    public void createTeam(Team team) {
        teamDao.save(team);
    }

    public void updateTeam(Team match) {
        teamDao.update(match);
    }

    public void deleteTeam(String id) {
        teamDao.delete(id);
    }
}
