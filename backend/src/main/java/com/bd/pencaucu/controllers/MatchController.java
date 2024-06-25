package com.bd.pencaucu.controllers;

import com.bd.pencaucu.models.Match;
import com.bd.pencaucu.models.dto.MatchDTO;
import com.bd.pencaucu.services.MatchService;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
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
    @Cacheable(value = "matches", key = "#id")
    public ResponseEntity<MatchDTO> getMatchById(@PathVariable String id) {
        MatchDTO match = matchService.getMatchById(id);

        if (match != null) {
            return new ResponseEntity<>(match, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @GetMapping
    public ResponseEntity<List<MatchDTO>> getAllMatches() {
        List<MatchDTO> matches = matchService.getAllMatches();

        if (matches == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!matches.isEmpty()) {
            return new ResponseEntity<>(matches, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @PostMapping
    @PreAuthorize("hasAuthority('ROLE_ADMIN')")
    public ResponseEntity<Match> createMatch(@RequestBody Match match) {
        matchService.createMatch(match);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

    @PutMapping
    @PreAuthorize("hasAuthority('ROLE_ADMIN')")
    public ResponseEntity<Match> updateMatch(@RequestBody Match match) {
        matchService.updateMatch(match);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping("/{id}")
    @PreAuthorize("hasAuthority('ROLE_ADMIN')")
    public ResponseEntity<Match> deleteMatch(@PathVariable String id) {
        matchService.deleteMatch(id);
        return new ResponseEntity<>(HttpStatus.OK);
    }
}
