package com.bd.pencaucu.controllers;

import com.bd.pencaucu.exceptions.ResourceNotFoundException;
import com.bd.pencaucu.models.Career;
import com.bd.pencaucu.services.CareerService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/careers")
public class CareerController {

    private final CareerService careerService;

    public CareerController(CareerService careerService) {
        this.careerService = careerService;
    }

    @GetMapping
    public ResponseEntity<List<Career>> getAllCareers() {
        List<Career> careers = careerService.getAllCareers();

        if (careers == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!careers.isEmpty()) {
            return new ResponseEntity<>(careers, HttpStatus.OK);
        }

        throw new ResourceNotFoundException("No careers found");
    }

    @PostMapping
    public ResponseEntity<String> createCareer(@RequestBody Career career) {
        careerService.createCareer(career);
        return new ResponseEntity<>(HttpStatus.CREATED);
    }

    @PutMapping
    public ResponseEntity<Career> updateCareer(@RequestBody Career career) {
        careerService.updateCareer(career);

        return new ResponseEntity<>(HttpStatus.OK);
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Career> deleteCareer(@PathVariable String id) {
        careerService.deleteCareer(id);

        return new ResponseEntity<>(HttpStatus.OK);
    }
}
