package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.domain.models.Admin;

import java.util.List;

public interface AdminDao {
    Admin findById(String id);
    List<Admin> findAll();
    void save(Admin admin);
    void update(Admin admin);
    void delete(String id);
}
