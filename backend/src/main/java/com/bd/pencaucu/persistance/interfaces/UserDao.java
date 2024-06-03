package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.domain.models.User;

import java.util.List;

public interface UserDao {
    User findById(String id);
    List<User> findAll();
    void save(User user);
    void update(User user);
    void delete(String id);
}
