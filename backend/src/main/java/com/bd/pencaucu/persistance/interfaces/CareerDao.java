package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.Career;

import java.util.List;

public interface CareerDao {
    List<Career> findAll();
    void save(Career career);
    void update(Career career);
    void delete(String id);
}
