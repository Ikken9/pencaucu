package com.bd.pencaucu.services;

import com.bd.pencaucu.models.Result;
import com.bd.pencaucu.models.dto.MatchDTO;
import com.bd.pencaucu.persistance.interfaces.ResultDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class ResultService {

    private final ResultDao resultDao;

    public Result findResultsById(int matchId) {
        return resultDao.findById(matchId);
    }

    public List<MatchDTO> findAllPendingResults() { return resultDao.findAllPending(); }

    public List<MatchDTO> findAllSubmittedResults() { return resultDao.findAllSubmitted(); }

    public List<Result> findAllResults() {
        return resultDao.findAll();
    }

    public void submitResult(Result result) {
        resultDao.save(result);
    }

    public void updateResult(Result result) {
        resultDao.update(result);
    }

    public void deleteResult(int id) {
        resultDao.delete(id);
    }
}
