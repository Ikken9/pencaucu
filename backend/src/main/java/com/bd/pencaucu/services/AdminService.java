package com.bd.pencaucu.services;

import com.bd.pencaucu.domain.models.Admin;
import com.bd.pencaucu.persistance.interfaces.AdminDao;
import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class AdminService {
    private final AdminDao adminDao;

    public List<Admin> getAllAdmins() {
        return adminDao.findAll();
    }

    public Admin getAdminById(String id) {
        return adminDao.findById(id);
    }

    public void createUser(Admin admin) { adminDao.save(admin); }

    public void updateUser(Admin admin) { adminDao.update(admin); }

    public void deleteUser(String id) { adminDao.delete(id); }
}
