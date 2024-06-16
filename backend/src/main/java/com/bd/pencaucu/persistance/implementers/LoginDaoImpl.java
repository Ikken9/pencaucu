package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.models.Login;
import com.bd.pencaucu.mappers.models.LoginMapper;
import com.bd.pencaucu.persistance.interfaces.LoginDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@RequiredArgsConstructor
public class LoginDaoImpl implements LoginDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public Login findById(String id) {
        String sql = "SELECT user_email, password FROM Logins WHERE user_email = ?";

        List<Login> result = jdbcTemplate.query(sql, new LoginMapper(), id);

        if (!result.isEmpty()) {
            return result.get(0);
        }

        String LOGIN_NOT_FOUND_MSG = "Login with email %s not found";
        throw new UsernameNotFoundException(String.format(LOGIN_NOT_FOUND_MSG, id));
    }

    @Override
    public void save(Login login) {
        String sql = "INSERT INTO Logins (user_email, password) VALUES (?, ?)";

        jdbcTemplate.update(sql,
                login.getEmail(),
                login.getPassword());
    }

    @Override
    public void update(Login login) {

    }

    @Override
    public void delete(String id) {
        String sql = "DELETE FROM Logins WHERE id = ?";
        jdbcTemplate.update(sql, id);
    }
}
