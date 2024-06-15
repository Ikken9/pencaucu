package com.bd.pencaucu.controllers;

import com.bd.pencaucu.domain.models.Login;
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
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;


@RestController
@RequestMapping("/auth")
public class AuthController {

    private final Argon2PasswordEncoder passwordEncoder = Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8();
    private final UserService userService;

    public AuthController(UserService userService) {
        this.userService = userService;
    }

    @PostMapping("/register")
    public ResponseEntity<String> registerUser(@RequestBody User user) throws InvalidUserRegistrationException {
        if (!user.getEmail().endsWith("@ucu.edu.uy")
                && !user.getEmail().endsWith("@correo.ucu.edu.uy")) {
            String domain = user.getEmail().split("@")[1];
            return new ResponseEntity<>(
                    String.format("Email domain %s is not valid.", domain),
                    HttpStatus.BAD_REQUEST);
        }
        user.setPassword(passwordEncoder.encode(user.getPassword()));
        userService.createUser(user);
        return new ResponseEntity<>(
                String.format("User with email %s has been registered.", user.getEmail()),
                HttpStatus.CREATED);
    }

    @PostMapping("/login")
    public ResponseEntity<String> logInUser(@RequestBody Login login) {
        UserDetails user = userService.loadUserByUsername(login.getEmail());
        if (user != null && passwordEncoder.matches(login.getPassword(), user.getPassword())) {
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