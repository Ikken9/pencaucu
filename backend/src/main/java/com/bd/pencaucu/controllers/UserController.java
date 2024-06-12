package com.bd.pencaucu.controllers;

import com.bd.pencaucu.domain.models.LogInRequest;
import com.bd.pencaucu.domain.models.User;
import com.bd.pencaucu.exceptions.InvalidUserRegistrationException;
import com.bd.pencaucu.services.UserService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController("/users")
public class UserController {

    private final UserService userService;

    private final Argon2PasswordEncoder passwordEncoder = Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8();

    public UserController(UserService userService) {
        this.userService = userService;
    }

    @GetMapping("{id}")
    public ResponseEntity<UserDetails> getUserById(@RequestBody @PathVariable String id) {
        UserDetails user = userService.loadUserByUsername(id);

        if (user != null) {
            return new ResponseEntity<>(user, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @GetMapping()
    public ResponseEntity<List<User>> getAllUsers() {
        List<User> users = userService.getAllUsers();

        if (users == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!users.isEmpty()) {
            return new ResponseEntity<>(users, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @PostMapping()
    public ResponseEntity<User> createUser(@RequestBody User user) {
        userService.createUser(user);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

    @PutMapping()
    public ResponseEntity<User> updateUser(@RequestBody User user) {
        userService.updateUser(user);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping("{id}")
    public ResponseEntity<User> deleteUser(@PathVariable String id) {
        userService.deleteUser(id);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @PostMapping("/register")
    public ResponseEntity<String> registerUser(@RequestBody User user) throws InvalidUserRegistrationException {
        if (!user.getEmail().endsWith("@ucu.edu.uy")
                || !user.getEmail().endsWith("@correo.ucu.edu.uy")) {
            return new ResponseEntity<>("Invalid email domain.",
                    HttpStatus.BAD_REQUEST);
        }
        user.setPassword(passwordEncoder.encode(user.getPassword()));
        userService.createUser(user);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @PostMapping("/login")
    public ResponseEntity<String> logInUser(@RequestBody LogInRequest logInRequest) {
        UserDetails user = userService.loadUserByUsername(logInRequest.getEmail());
        if (user != null && passwordEncoder.matches(logInRequest.getPassword(), user.getPassword())) {
            Authentication authentication = new UsernamePasswordAuthenticationToken(
                    user.getUsername(),
                    user.getPassword()
            );
            SecurityContextHolder.getContext().setAuthentication(authentication);
            return new ResponseEntity<>(
                    "Login successful!",
                    HttpStatus.OK);
        } else {
            return new ResponseEntity<>(
                    "Invalid username or password.",
                    HttpStatus.UNAUTHORIZED);
        }
    }
}
