package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.User;
import com.bd.pencaucu.models.dto.UserDTO;

import java.util.List;

public interface UserDao {
    UserDTO findById(String id);
    List<UserDTO> findAll();
    void save(User user);
    void update(User user);
    void delete(String id);
}
