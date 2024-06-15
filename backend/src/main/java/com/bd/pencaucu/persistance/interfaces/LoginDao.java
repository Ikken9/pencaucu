package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.domain.models.Login;

public interface LoginDao {

    Login findById(String id);
    void save(Login login);
    void update(Login login);
    void delete(String id);
}
