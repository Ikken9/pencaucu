package com.bd.pencaucu.services;

import com.bd.pencaucu.domain.models.Player;
import com.bd.pencaucu.persistance.interfaces.PlayerDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class PlayerService {
    private final PlayerDao playerDao;

    public List<Player> getAllPlayers() {
        return playerDao.findAll();
    }

    public Player getPlayerById(String id) {
        return playerDao.findById(id);
    }

    public void createPlayer(Player player) {
        playerDao.save(player);
    }

    public void updatePlayer (Player player) {
        playerDao.update(player);
    }

    public void deletePlayer(String id) {
        playerDao.delete(id);
    }
}
