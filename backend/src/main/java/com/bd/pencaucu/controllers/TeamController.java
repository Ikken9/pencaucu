package com.bd.pencaucu.controllers;

import com.bd.pencaucu.domain.models.Team;

import com.bd.pencaucu.services.TeamService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/teams")
public class TeamController {

    private final TeamService teamService;

    public TeamController(TeamService teamService) {
        this.teamService = teamService;
    }

    @GetMapping("/{id}")
    public ResponseEntity<Team> getTeamById(@RequestBody @PathVariable String id) {
        Team team = teamService.getTeamById(id);

        if (team != null) {
            return new ResponseEntity<>(team, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @GetMapping
    public ResponseEntity<List<Team>> getAllTeams() {
        List<Team> teams = teamService.getAllTeams();

        if (teams == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!teams.isEmpty()) {
            return new ResponseEntity<>(teams, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @PostMapping
    public ResponseEntity<Team> createTeam(@RequestBody Team team) {
        teamService.createTeam(team);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

    @PutMapping
    public ResponseEntity<Team> updateTeam(@RequestBody Team team) {
        teamService.updateTeam(team);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Team> deleteTeam(@PathVariable String id) {
        teamService.deleteTeam(id);
        return new ResponseEntity<>(HttpStatus.OK);
    }
}
