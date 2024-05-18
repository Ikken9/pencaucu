package com.bd.pencaucu.controllers;

import com.bd.pencaucu.domain.models.Match;
import com.bd.pencaucu.services.MatchService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/matches")
public class MatchController {

    private final MatchService matchService;

    public MatchController(MatchService matchService) {
        this.matchService = matchService;
    }

    @GetMapping("/{id}")
    public ResponseEntity<Match> getMatchById(@PathVariable String id) {
        Match match = matchService.getMatchById(id);

        if (match != null) {
            return new ResponseEntity<>(match, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @GetMapping
    public ResponseEntity<List<Match>> getAllMatches() {
        List<Match> matches = matchService.getAllMatches();

        if (matches == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!matches.isEmpty()) {
            return new ResponseEntity<>(matches, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @PostMapping
    public ResponseEntity<Match> createMatch(@RequestBody Match match) {
        matchService.createMatch(match);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

    @PutMapping
    public ResponseEntity<Match> updateMatch(@RequestBody Match match) {
        matchService.updateMatch(match);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Match> deleteMatch(@PathVariable String id) {
        matchService.deleteMatch(id);
        return new ResponseEntity<>(HttpStatus.OK);
    }
}
