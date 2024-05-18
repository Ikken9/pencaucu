package com.bd.pencaucu.controllers;

import com.bd.pencaucu.domain.models.Stadium;
import com.bd.pencaucu.services.StadiumService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/stadiums")
public class StadiumController {

    private final StadiumService stadiumService;

    public StadiumController(StadiumService stadiumService) {
        this.stadiumService = stadiumService;
    }

    @GetMapping("/{id}")
    public ResponseEntity<Stadium> getStadiumById(@RequestBody @PathVariable String id) {
        Stadium stadium = stadiumService.getStadiumById(id);

        if (stadium != null) {
            return new ResponseEntity<>(stadium, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @GetMapping
    public ResponseEntity<List<Stadium>> getAllStadiums() {
        List<Stadium> stadiums = stadiumService.getAllStadiums();

        if (stadiums == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!stadiums.isEmpty()) {
            return new ResponseEntity<>(stadiums, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @PostMapping
    public ResponseEntity<Stadium> createStadium(@RequestBody Stadium stadium) {
        stadiumService.createStadium(stadium);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

    @PutMapping
    public ResponseEntity<Stadium> updateStadium(@RequestBody Stadium stadium) {
        stadiumService.updateStadium(stadium);
        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Stadium> deleteStadium(@PathVariable String id) {
        stadiumService.deleteStadium(id);
        return new ResponseEntity<>(HttpStatus.OK);
    }
}
