package com.bd.pencaucu.controllers;

import com.bd.pencaucu.domain.models.Result;
import com.bd.pencaucu.services.ResultService;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController("/results")
public class ResultController {

    private final ResultService resultService;

    public ResultController(ResultService resultService) {
        this.resultService = resultService;
    }

    @GetMapping("{id}")
    public ResponseEntity<List<Result>> getResultsByMatchId(@PathVariable int id) {
        List<Result> results = resultService.findResultsByMatchId(id);

        return new ResponseEntity<>(results, HttpStatus.OK);
    }

    @GetMapping("{teamName}")
    public ResponseEntity<List<Result>> getResultsByTeamName(@PathVariable String teamName) {
        List<Result> results = resultService.finResultsByTeamName(teamName);

        return new ResponseEntity<>(results, HttpStatus.OK);
    }

    @PostMapping()
    public ResponseEntity<String> submitResult(@RequestBody Result result) {
        resultService.submitResult(result);

        return new ResponseEntity<>(
                String.format(
                        "Result of team %s and match id %d submitted successfully",
                        result.getTeamName(),
                        result.getMatchId()
                ),
                HttpStatus.OK);
    }

    @PutMapping()
    public ResponseEntity<Result> updateResult(@RequestBody Result result) {
        resultService.updateResult(result);

        return new ResponseEntity<>(result, HttpStatus.OK);
    }

    @DeleteMapping()
    public ResponseEntity<Result> deleteResult(@RequestBody Result result) {
        resultService.deleteResult(result);

        return new ResponseEntity<>(HttpStatus.OK);
    }

}
