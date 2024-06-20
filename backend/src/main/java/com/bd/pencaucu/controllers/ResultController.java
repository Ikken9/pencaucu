package com.bd.pencaucu.controllers;

import com.bd.pencaucu.models.Result;
import com.bd.pencaucu.services.ResultService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import java.util.List;

@RestController
@RequestMapping("/results")
public class ResultController {

    private final ResultService resultService;

    public ResultController(ResultService resultService) {
        this.resultService = resultService;
    }

    @GetMapping
    public ResponseEntity<List<Result>> getAllResults() {
        List<Result> results = resultService.findAllResults();

        if (results == null) {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        } else if (!results.isEmpty()) {
            return new ResponseEntity<>(results, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @GetMapping("/{id}")
    public ResponseEntity<Result> getResultById(@PathVariable int id) {
        Result result = resultService.findResultsById(id);

        if (result != null) {
            return new ResponseEntity<>(result, HttpStatus.OK);
        }

        return new ResponseEntity<>(HttpStatus.NOT_FOUND);
    }

    @PostMapping
    public ResponseEntity<Result> createResult(@RequestBody Result result) {
        resultService.submitResult(result);
        return new ResponseEntity<>(result, HttpStatus.CREATED);
    }

    @PutMapping
    public ResponseEntity<Result> updateResult(@RequestBody Result result) {
        resultService.updateResult(result);
        return new ResponseEntity<>(result, HttpStatus.OK);
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Result> deleteResultById(@PathVariable int id) {
        resultService.deleteResult(id);
        return new ResponseEntity<>(HttpStatus.OK);
    }
}
