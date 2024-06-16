package com.bd.pencaucu.services;

import com.bd.pencaucu.models.Result;
import com.bd.pencaucu.persistance.interfaces.ResultDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class ResultService {

    private final ResultDao resultDao;

    public List<Result> findResultsByMatchId(int matchId) {
        return resultDao.findByMatchId(matchId);
    }

    public List<Result> finResultsByTeamName(String teamName) {
        return resultDao.findByTeamName(teamName);
    }

    public void submitResult(Result result) {
        resultDao.save(result);
    }

    public void updateResult(Result result) {
        resultDao.update(result);
    }

    public void deleteResult(Result result) {
        resultDao.delete(result);
    }
}
