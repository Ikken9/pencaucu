package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.Match;
import com.bd.pencaucu.models.dto.MatchDTO;
import java.util.List;

public interface MatchDao {
    MatchDTO findById(String id);
    List<MatchDTO> findAll();
    void save(Match match);
    void update(Match match);
    void delete(String id);
}
