package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.User;

import java.util.List;

public interface UserDao {
    User findById(String id);
    List<User> findAll();
    void save(User user);
    void update(User user);
    void delete(String id);
}
