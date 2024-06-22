package com.bd.pencaucu.controllers;

import com.bd.pencaucu.exceptions.ResourceNotFoundException;
import com.bd.pencaucu.models.Stage;
import com.bd.pencaucu.services.StageService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/stages")
public class StageController {

    private final StageService stageService;

    public StageController(StageService stageService) {
        this.stageService = stageService;
    }

    @GetMapping
    public ResponseEntity<List<Stage>> getAllStages() {
        List<Stage> stages = stageService.getAllStages();

        if (stages == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!stages.isEmpty()) {
            return new ResponseEntity<>(stages, HttpStatus.OK);
        }

        throw new ResourceNotFoundException("No stages found");
    }

}
