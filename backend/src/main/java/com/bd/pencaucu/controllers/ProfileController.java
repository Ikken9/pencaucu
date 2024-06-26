package com.bd.pencaucu.controllers;

import com.bd.pencaucu.models.dto.ProfileDTO;
import com.bd.pencaucu.services.ProfileService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/profile")
public class ProfileController {

    private final ProfileService profileService;

    public ProfileController(ProfileService profileService) {
        this.profileService = profileService;
    }

    @GetMapping("/{profileEmail}")
    public ResponseEntity<ProfileDTO> getProfileByEmail(@PathVariable String profileEmail) {
        if (!userHaveAccess(profileEmail)) {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }

        ProfileDTO profile = profileService.getProfileByEmail(profileEmail);

        if (profile != null) {
            return new ResponseEntity<>(profile, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    private boolean userHaveAccess(String id) {
        String user = SecurityContextHolder.getContext().getAuthentication().getName();
        return user.equals(id);
    }

}
