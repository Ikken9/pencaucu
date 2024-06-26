package com.bd.pencaucu.controllers;

import com.bd.pencaucu.models.Player;
import com.bd.pencaucu.models.dto.PlayerDTO;
import com.bd.pencaucu.services.PlayerService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/players")
public class PlayerController {

    private final PlayerService playerService;

    public PlayerController(PlayerService playerService) {
        this.playerService = playerService;
    }

    @GetMapping("/{id}")
    public ResponseEntity<PlayerDTO> getPlayerById(@RequestBody @PathVariable String id) {
        if (!userHaveAccess(id)) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }

        PlayerDTO player = playerService.getPlayerById(id);

        if (player != null) {
            return new ResponseEntity<>(player, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @GetMapping
    public ResponseEntity<List<PlayerDTO>> getAllPlayers() {
        List<PlayerDTO> players = playerService.getAllPlayers();

        if (players == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!players.isEmpty()) {
            return new ResponseEntity<>(players, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @PostMapping
    @PreAuthorize("hasAuthority('ROLE_ADMIN')")
    public ResponseEntity<Player> createPlayer(@RequestBody Player player) {
        if (!userHaveAccess(player.getEmail())) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }
        playerService.createPlayer(player);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

    @PutMapping
    @PreAuthorize("hasAuthority('ROLE_ADMIN')")
    public ResponseEntity<Player> updatePlayer(@RequestBody Player player) {
        if (!userHaveAccess(player.getEmail())) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }

        playerService.updatePlayer(player);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping("/{id}")
    @PreAuthorize("hasAuthority('ROLE_ADMIN')")
    public ResponseEntity<Player> deletePlayer(@PathVariable String id) {
        if (!userHaveAccess(id)) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }

        playerService.deletePlayer(id);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    private boolean userHaveAccess(String id) {
        String user = SecurityContextHolder.getContext().getAuthentication().getName();
        return user.equals(id);
    }
}
