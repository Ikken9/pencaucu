package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.Result;

import java.util.List;

public interface ResultDao {
    List<Result> findByMatchId(int matchId);
    List<Result> findByTeamName(String teamName);
    void save(Result result);
    void update(Result result);
    void delete(Result result);
}
