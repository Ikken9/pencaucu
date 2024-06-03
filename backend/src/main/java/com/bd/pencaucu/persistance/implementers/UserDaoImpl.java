package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.User;
import com.bd.pencaucu.persistance.interfaces.UserDao;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public class UserDaoImpl implements UserDao {
    @Override
    public User findById(String id) {
        return null;
    }

    @Override
    public List<User> findAll() {
        return List.of();
    }

    @Override
    public void save(User user) {

    }

    @Override
    public void update(User user) {

    }

    @Override
    public void delete(String id) {

    }
}
