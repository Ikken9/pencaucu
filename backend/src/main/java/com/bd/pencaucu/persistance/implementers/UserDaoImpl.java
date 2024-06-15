package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.User;
import com.bd.pencaucu.mappers.models.UserMapper;
import com.bd.pencaucu.persistance.interfaces.UserDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@RequiredArgsConstructor
public class UserDaoImpl implements UserDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public User findById(String id) throws UsernameNotFoundException {
        String sql = "SELECT email, name FROM Users WHERE email = ?";
        List<User> users = jdbcTemplate.query(sql, new UserMapper(), id);

        if (!users.isEmpty()) {
            return users.get(0);
        }

        String USER_NOT_FOUND_MSG = "User with email %s not found";
        throw new UsernameNotFoundException(String.format(USER_NOT_FOUND_MSG, id));
    }

    @Override
    public List<User> findAll() {
        String sql = "SELECT email, name FROM Users";

        return jdbcTemplate.query(sql, new UserMapper());
    }

    @Override
    public void save(User user) {
        String sql = "INSERT INTO Users (email, name) VALUES (?, ?)";

        jdbcTemplate.update(sql,
                user.getEmail(),
                user.getName());
    }

    @Override
    public void update(User user) {

    }

    @Override
    public void delete(String id) {
        String sql = "DELETE FROM Users WHERE email = ?";
        jdbcTemplate.update(sql, id);
    }
}
