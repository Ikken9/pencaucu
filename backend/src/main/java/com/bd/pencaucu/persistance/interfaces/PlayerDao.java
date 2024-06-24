package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.Player;
import com.bd.pencaucu.models.dto.PlayerDTO;

import java.util.List;

public interface PlayerDao {
    PlayerDTO findById(String username);
    List<PlayerDTO> findAll();
    List<String> findAllPlayersEmails();
    void save(Player player);
    void update(Player player);
    void delete(String id);
}
