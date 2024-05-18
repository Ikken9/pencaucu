package com.bd.pencaucu.services;

import com.bd.pencaucu.domain.models.Match;
import com.bd.pencaucu.persistance.interfaces.MatchDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class MatchService {

    private final MatchDao matchDao;

    public List<Match> getAllMatches() {
        return matchDao.findAll();
    }

    public Match getMatchById(String id) {
        return matchDao.findById(id);
    }

    public void createMatch(Match match) {
        matchDao.save(match);
    }

    public void updateMatch(Match match) {
        matchDao.update(match);
    }

    public void deleteMatch(String id) {
        matchDao.delete(id);
    }
}
