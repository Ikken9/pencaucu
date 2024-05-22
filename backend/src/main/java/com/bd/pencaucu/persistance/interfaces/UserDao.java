package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.domain.models.Player;

import java.util.List;

public interface UserDao {
    Player findById(String id);
    List<Player> findAll();
    void save(Player player);
    void update(Player player);
    void delete(String id);
}
