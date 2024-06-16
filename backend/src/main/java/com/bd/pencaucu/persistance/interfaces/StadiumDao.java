package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.Stadium;

import java.util.List;

public interface StadiumDao {
    Stadium findById(String id);
    List<Stadium> findAll();
    void save(Stadium stadium);
    void update(Stadium stadium);
    void delete(String id);
}
