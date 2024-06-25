package com.bd.pencaucu.controllers;

import com.bd.pencaucu.models.Bet;
import com.bd.pencaucu.services.BetService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController()
@RequestMapping("/bets")
public class BetController {

    private final BetService betService;

    public BetController(BetService betService) {
        this.betService = betService;
    }

    @GetMapping("/{playerEmail}/{matchId}")
    public ResponseEntity<Bet> getBetsById(@PathVariable String playerEmail, @PathVariable int matchId) {
        if (!userHaveAccess(playerEmail)) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }

        Bet bet = betService.getBetById(playerEmail, matchId);

        if (bet == null) {
            return new ResponseEntity<>(HttpStatus.NOT_FOUND);
        }

        return new ResponseEntity<>(bet, HttpStatus.OK);
    }

    @GetMapping("/{playerEmail}")
    public ResponseEntity<List<Bet>> getPlayerBetsById(@PathVariable String playerEmail) {
        if (!userHaveAccess(playerEmail)) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }

        List<Bet> bet = betService.getPlayerBetsById(playerEmail);

        if (bet == null) {
            return new ResponseEntity<>(HttpStatus.NOT_FOUND);
        }

        return new ResponseEntity<>(bet, HttpStatus.OK);
    }

    @PostMapping
    @PreAuthorize("hasAuthority('ROLE_PLAYER')")
    public ResponseEntity<Bet> submitBet(@RequestBody Bet bet) {
        if (!userHaveAccess(bet.getPlayerEmail())) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }

        betService.submitBet(bet);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

    @PutMapping
    @PreAuthorize("hasAuthority('ROLE_PLAYER')")
    public ResponseEntity<Bet> updateBet(@RequestBody Bet bet) {
        if (!userHaveAccess(bet.getPlayerEmail())) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }

        betService.updateBet(bet);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping
    @PreAuthorize("hasAuthority('ROLE_PLAYER')")
    public ResponseEntity<Bet> deleteBet(@RequestBody Bet bet) {
        if (!userHaveAccess(bet.getPlayerEmail())) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }

        betService.deleteBet(bet);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    private boolean userHaveAccess(String id) {
        String user = SecurityContextHolder.getContext().getAuthentication().getName();
        return user.equals(id);
    }
}
