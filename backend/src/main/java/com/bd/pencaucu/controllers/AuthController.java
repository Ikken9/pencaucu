package com.bd.pencaucu.controllers;

import com.bd.pencaucu.config.TokenProvider;
import com.bd.pencaucu.models.Login;
import com.bd.pencaucu.models.Player;
import com.bd.pencaucu.models.User;
import com.bd.pencaucu.exceptions.InvalidUserRegistrationException;
import com.bd.pencaucu.services.LoginService;
import com.bd.pencaucu.services.PlayerService;
import com.bd.pencaucu.services.UserService;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.AuthenticationException;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;


@RestController
@RequestMapping("")
public class AuthController {

    private final PasswordEncoder passwordEncoder;
    private final AuthenticationManager authenticationManager;
    private final UserService userService;
    private final LoginService loginService;
    private final PlayerService playerService;
    private final TokenProvider tokenProvider;

    public AuthController(UserService userService,
                          LoginService loginService,
                          PlayerService playerService,
                          PasswordEncoder passwordEncoder,
                          AuthenticationManager authenticationManager, TokenProvider tokenProvider) {
        this.userService = userService;
        this.loginService = loginService;
        this.playerService = playerService;
        this.passwordEncoder = passwordEncoder;
        this.authenticationManager = authenticationManager;
        this.tokenProvider = tokenProvider;
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
        Login login = new Login();
        login.setEmail(user.getEmail());
        login.setPassword(passwordEncoder.encode(user.getPassword()));

        Player player = new Player();
        player.setEmail(user.getEmail());
        player.setCareerName(user.getCareer());
        player.setProfilePictureUrl(user.getProfilePictureUrl());

        userService.createUser(user);
        loginService.saveLoginUser(login);
        playerService.createPlayer(player);

        return new ResponseEntity<>(
                String.format("User with email %s has been registered.", user.getEmail()),
                HttpStatus.CREATED);
    }

    @PostMapping("/login")
    public ResponseEntity<String> logInUser(@RequestBody Login login) throws AuthenticationException {
        Authentication auth = new UsernamePasswordAuthenticationToken(login.getEmail(), login.getPassword());
        Authentication authUser = authenticationManager.authenticate(auth);
        String accessToken = tokenProvider.generateAccessToken((Login) authUser.getPrincipal());

        HttpHeaders httpHeaders = new HttpHeaders();
        httpHeaders.set("Authorization", "Bearer " + accessToken);
        return new ResponseEntity<>("Login successful!", httpHeaders, HttpStatus.OK);
    }
}
