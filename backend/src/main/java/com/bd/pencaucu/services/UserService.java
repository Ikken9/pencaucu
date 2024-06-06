package com.bd.pencaucu.services;

import com.bd.pencaucu.domain.models.User;
import com.bd.pencaucu.persistance.interfaces.UserDao;
import lombok.RequiredArgsConstructor;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class UserService implements UserDetailsService {

    private final UserDao userDao;

    @Override
    public UserDetails loadUserByUsername(String username) throws UsernameNotFoundException {
        return userDao.findById(username);
    }

    public List<User> getAllUsers() {
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
