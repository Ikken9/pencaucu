package com.bd.pencaucu.controllers;

import com.bd.pencaucu.models.User;
import com.bd.pencaucu.models.dto.UserDTO;
import com.bd.pencaucu.services.UserService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.security.core.context.SecurityContextHolder;
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
    public ResponseEntity<UserDTO> getUserById(@PathVariable String id) {
        if (!userHaveAccess(id)) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }

        UserDTO user = userService.findUserById(id);

        return new ResponseEntity<>(user, HttpStatus.OK);
    }

    @GetMapping()
    public ResponseEntity<List<UserDTO>> getAllUsers() {
        List<UserDTO> users = userService.getAllUsers();

        if (users == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!users.isEmpty()) {
            return new ResponseEntity<>(users, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @PutMapping()
    public ResponseEntity<User> updateUser(@RequestBody User user) {
        if (!userHaveAccess(user.getEmail())) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }

        userService.updateUser(user);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping("/{id}")
    @PreAuthorize("hasAuthority('ROLE_ADMIN')")
    public ResponseEntity<User> deleteUser(@PathVariable String id) {
        userService.deleteUser(id);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    private boolean userHaveAccess(String id) {
        String user = SecurityContextHolder.getContext().getAuthentication().getName();
        return user.equals(id);
    }
}
