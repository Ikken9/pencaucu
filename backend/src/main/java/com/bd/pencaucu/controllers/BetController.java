package com.bd.pencaucu.controllers;

import com.bd.pencaucu.models.Bet;
import com.bd.pencaucu.services.BetService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController()
@RequestMapping("/bets")
public class BetController {

    private final BetService betService;

    public BetController(BetService betService) {
        this.betService = betService;
    }

    @GetMapping("/{username}")
    public ResponseEntity<List<Bet>> getBetsById(@PathVariable String username) {
        List<Bet> bets = betService.getBetsById(username);

        if (bets == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!bets.isEmpty()) {
            return new ResponseEntity<>(bets, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @PostMapping
    public ResponseEntity<Bet> submitBet(@RequestBody Bet bet) {
        betService.submitBet(bet);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

    @PutMapping
    public ResponseEntity<Bet> updateBet(@RequestBody Bet bet) {
        betService.updateBet(bet);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping
    public ResponseEntity<Bet> deleteBet(@RequestBody Bet bet) {
        betService.deleteBet(bet);
        return new ResponseEntity<>(HttpStatus.OK);
    }
}
