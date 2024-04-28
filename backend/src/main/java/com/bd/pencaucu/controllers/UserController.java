package com.bd.pencaucu.controllers;

import com.bd.pencaucu.domain.models.User;
import com.bd.pencaucu.persistance.interfaces.UserDao;
import com.bd.pencaucu.services.UserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/users")
public class UserController {

    @Autowired
    private UserService userService;

    public UserController(UserService userService) {
        this.userService = userService;
    }

    @GetMapping("/{id}")
    public ResponseEntity<User> getUserById(@RequestBody @PathVariable String id) {

    }

    @GetMapping
    public ResponseEntity<List<User>> getAllUsers() {

    }

    @PostMapping
    public ResponseEntity<User> createUser(@RequestBody UserDao user) {

    }

    @PutMapping
    public ResponseEntity<User> updateUser(@RequestBody UserDao user) {

    }

    @DeleteMapping("/{id}")
    public ResponseEntity<User> deleteUser(@PathVariable String id) {
        
    }
}
