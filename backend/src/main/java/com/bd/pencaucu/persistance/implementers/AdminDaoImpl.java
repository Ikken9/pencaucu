package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.Admin;
import com.bd.pencaucu.persistance.interfaces.AdminDao;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public class AdminDaoImpl implements AdminDao {

    JdbcTemplate jdbcTemplate;

    @Override
    public Admin findById(String id) {
        return null;
    }

    @Override
    public List<Admin> findAll() {
        return List.of();
    }

    @Override
    public void save(Admin admin) {

    }

    @Override
    public void update(Admin admin) {

    }

    @Override
    public void delete(String id) {

    }
}
