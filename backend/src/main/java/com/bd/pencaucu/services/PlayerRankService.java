package com.bd.pencaucu.services;

import com.bd.pencaucu.models.Rank;
import com.bd.pencaucu.models.dto.RankDTO;
import com.bd.pencaucu.persistance.interfaces.RankDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class PlayerRankService {

    private final RankDao rankDao;

    public List<RankDTO> getPlayerRanks(String id) { return rankDao.findPlayerRanks(id); }

    public void savePlayerRank(Rank rank) { rankDao.save(rank); }
}
