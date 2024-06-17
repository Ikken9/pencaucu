package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.Login;
import org.springframework.security.core.userdetails.UserDetails;

public interface LoginDao {
    UserDetails findById(String id);
    void save(Login login);
    void update(Login login);
    void delete(String id);
}
