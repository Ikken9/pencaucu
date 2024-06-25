package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.Rank;
import com.bd.pencaucu.models.dto.RankDTO;

import java.util.List;

public interface RankDao {
    List<RankDTO> findPlayerRanks(String id);
    void save(Rank rank);
}
