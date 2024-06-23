package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.Bet;
import java.util.List;

public interface BetDao {
    Bet findById(String playerEmail, int matchId);
    List<Bet> findPlayerBetsById(String playerEmail);
    void save(Bet bet);
    void update(Bet bet);
    void delete(Bet bet);
}
