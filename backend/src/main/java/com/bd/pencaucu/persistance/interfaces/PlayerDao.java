package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.domain.models.Player;
import com.bd.pencaucu.dto.PlayerDTO;

import java.util.List;

public interface PlayerDao {
    PlayerDTO findById(String username);
    List<PlayerDTO> findAll();
    void save(Player player);
    void update(Player player);
    void delete(String id);
}
