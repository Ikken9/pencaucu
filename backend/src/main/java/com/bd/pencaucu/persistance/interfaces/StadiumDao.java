package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.domain.models.Stadium;

import java.util.List;

public interface StadiumDao {
    Stadium findById(String id);
    List<Stadium> findAll();
    void save(Stadium team);
    void update(Stadium team);
    void delete(String id);
}
