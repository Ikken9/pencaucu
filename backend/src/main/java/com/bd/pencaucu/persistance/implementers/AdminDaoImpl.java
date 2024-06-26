package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.models.Admin;
import com.bd.pencaucu.mappers.models.AdminMapper;
import com.bd.pencaucu.persistance.interfaces.AdminDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@RequiredArgsConstructor
public class AdminDaoImpl implements AdminDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public Admin findById(String id) throws UsernameNotFoundException {
        String sql = "SELECT admin_email FROM Admins WHERE admin_email = ?";
        List<Admin> admins = jdbcTemplate.query(sql, new AdminMapper(), id);

        if (!admins.isEmpty()) {
            return admins.get(0);
        }

        String ADMIN_NOT_FOUND_MSG = "Admin with email %s not found";
        throw new UsernameNotFoundException(String.format(ADMIN_NOT_FOUND_MSG, id));
    }

    @Override
    public List<Admin> findAll() {
        String sql = "SELECT admin_email FROM Admins";

        return jdbcTemplate.query(sql, new AdminMapper());
    }

    @Override
    public void save(Admin admin) {
        String sql = "INSERT INTO Admins (admin_email) VALUES (?)";

        jdbcTemplate.update(sql, admin.getEmail());
    }

    @Override
    public void update(Admin admin) {
        String sql = "UPDATE Admins SET admin_email = ? WHERE admin_email = ?";

        jdbcTemplate.update(sql, admin.getEmail(), admin.getEmail());
    }

    @Override
    public void delete(String id) {
        String sql = "DELETE FROM Admins WHERE admin_email = ?";
        jdbcTemplate.update(sql, id);
    }
}
