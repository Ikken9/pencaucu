package com.bd.pencaucu.controllers;

import com.bd.pencaucu.domain.models.Player;
import com.bd.pencaucu.services.UserService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/users")
public class UserController {

    private final UserService userService;

    public UserController(UserService userService) {
        this.userService = userService;
    }

    @GetMapping("/{id}")
    public ResponseEntity<Player> getUserById(@RequestBody @PathVariable String id) {
        Player player = userService.getUserById(id);

        if (player != null) {
            return new ResponseEntity<>(player, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @GetMapping
    public ResponseEntity<List<Player>> getAllUsers() {
        List<Player> players = userService.getAllUsers();

        if (players == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!players.isEmpty()) {
            return new ResponseEntity<>(players, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @PostMapping
    public ResponseEntity<Player> createUser(@RequestBody Player player) {
        userService.createUser(player);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

    @PutMapping
    public ResponseEntity<Player> updateUser(@RequestBody Player player) {
        userService.updateUser(player);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Player> deleteUser(@PathVariable String id) {
        userService.deleteUser(id);
        return new ResponseEntity<>(HttpStatus.OK);
    }
}
