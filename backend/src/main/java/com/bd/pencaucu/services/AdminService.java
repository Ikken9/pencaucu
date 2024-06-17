package com.bd.pencaucu.services;

import com.bd.pencaucu.models.Admin;
import com.bd.pencaucu.persistance.implementers.AdminDaoImpl;
import com.bd.pencaucu.persistance.interfaces.AdminDao;
import lombok.RequiredArgsConstructor;
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

    public void createAdmin(Admin admin) { adminDao.save(admin); }

    public void updateAdmin(Admin admin) { adminDao.update(admin); }

    public void deleteAdmin(String id) { adminDao.delete(id); }
}
