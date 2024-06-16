package com.bd.pencaucu.services;

import com.bd.pencaucu.domain.models.Player;
import com.bd.pencaucu.dto.PlayerDTO;
import com.bd.pencaucu.persistance.interfaces.PlayerDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class PlayerService {
    private final PlayerDao playerDao;

    public List<PlayerDTO> getAllPlayers() {
        return playerDao.findAll();
    }

    public PlayerDTO getPlayerById(String username) {
        return playerDao.findById(username);
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
