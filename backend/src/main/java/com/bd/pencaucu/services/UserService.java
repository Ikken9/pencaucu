package com.bd.pencaucu.services;

import com.bd.pencaucu.models.User;
import com.bd.pencaucu.models.dto.UserDTO;
import com.bd.pencaucu.persistance.interfaces.UserDao;
import lombok.RequiredArgsConstructor;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class UserService {

    private final UserDao userDao;

    public UserDTO findUserById(String username) throws UsernameNotFoundException {
        return userDao.findById(username);
    }

    public List<UserDTO> getAllUsers() {
        return userDao.findAll();
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
