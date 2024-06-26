package com.bd.pencaucu.services;

import com.bd.pencaucu.models.Bet;
import com.bd.pencaucu.persistance.interfaces.BetDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class BetService {

    private final BetDao betDao;

    public Bet getBetById(String playerEmail, int matchId) { return betDao.findById(playerEmail, matchId); }

    public List<Bet> getPlayerBetsById(String playerEmail) {
        return betDao.findPlayerBetsById(playerEmail);
    }

    public void submitBet(Bet bet) { betDao.save(bet); }

    public void updateBet(Bet bet) { betDao.update(bet); }

    public void deleteBet(Bet bet) { betDao.delete(bet); }
}
