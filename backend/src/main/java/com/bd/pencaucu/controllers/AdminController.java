package com.bd.pencaucu.controllers;

import com.bd.pencaucu.domain.models.Admin;
import com.bd.pencaucu.services.AdminService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/admins")
public class AdminController {

    private final AdminService adminService;

    public AdminController(AdminService adminService) {
        this.adminService = adminService;
    }

    @GetMapping("/{id}")
    public ResponseEntity<Admin> getAdminById(@RequestBody @PathVariable String id) {
        Admin admin = adminService.getAdminById(id);

        if (admin != null) {
            return new ResponseEntity<>(admin, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @GetMapping
    public ResponseEntity<List<Admin>> getAllAdmins() {
        List<Admin> admins = adminService.getAllAdmins();

        if (admins == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!admins.isEmpty()) {
            return new ResponseEntity<>(admins, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @PostMapping
    public ResponseEntity<Admin> createUser(@RequestBody Admin admin) {
        adminService.createUser(admin);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

    @PutMapping
    public ResponseEntity<Admin> updateUser(@RequestBody Admin admin) {
        adminService.updateUser(admin);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Admin> deleteUser(@PathVariable String id) {
        adminService.deleteUser(id);
        return new ResponseEntity<>(HttpStatus.OK);
    }
}
