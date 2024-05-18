package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.domain.models.Match;

import java.util.List;

public interface MatchDao {
    Match findById(String id);
    List<Match> findAll();
    void save(Match match);
    void update(Match match);
    void delete(String id);
}
