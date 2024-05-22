package com.bd.pencaucu.services;

import com.bd.pencaucu.domain.models.Player;
import com.bd.pencaucu.persistance.interfaces.UserDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class UserService {

    private final UserDao userDao;

    public List<Player> getAllUsers() {
        return userDao.findAll();
    }

    public Player getUserById(String id) {
        return userDao.findById(id);
    }

    public void createUser(Player player) {
        userDao.save(player);
    }

    public void updateUser (Player player) {
        userDao.update(player);
    }

    public void deleteUser(String id) {
        userDao.delete(id);
    }
}
