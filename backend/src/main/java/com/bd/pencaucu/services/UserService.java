package com.bd.pencaucu.services;

import com.bd.pencaucu.domain.models.User;
import com.bd.pencaucu.persistance.interfaces.UserDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class UserService {

    private final UserDao userDao;

    public List<User> getAllUsers() {
        return userDao.findAll();
    }

    public User getUserById(String id) {
        return userDao.findById(id);
    }

    public void createUser(User user) {
        userDao.save(user);
    }

    public void updateUser (User user) {
        userDao.update(user);
    }

    public void deleteUser(String id) {
        userDao.delete(id);
    }
}
