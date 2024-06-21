package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.Bet;
import java.util.List;

public interface BetDao {
    List<Bet> findById(String username);
    void save(Bet bet);
    void update(Bet bet);
    void delete(Bet bet);
}
