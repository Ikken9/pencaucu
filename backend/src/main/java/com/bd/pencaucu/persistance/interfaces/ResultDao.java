package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.Result;
import com.bd.pencaucu.models.dto.MatchDTO;

import java.util.List;

public interface ResultDao {
    Result findById(int matchId);
    List<MatchDTO> findAllPending();
    List<MatchDTO> findAllSubmitted();
    List<Result> findAll();
    void save(Result result);
    void update(Result result);
    void delete(int id);
}
