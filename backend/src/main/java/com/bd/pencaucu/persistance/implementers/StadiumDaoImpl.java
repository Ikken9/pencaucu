package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.Stadium;
import com.bd.pencaucu.mappers.StadiumMapper;
import com.bd.pencaucu.persistance.interfaces.StadiumDao;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public class StadiumDaoImpl implements StadiumDao {
    
    private JdbcTemplate jdbcTemplate;
    
    @Override
    public Stadium findById(String id) {
        String sql = "SELECT * FROM Stadium WHERE id = ?";

        List<Stadium> result = jdbcTemplate.query(sql, new StadiumMapper(), id);

        if (!result.isEmpty()) {
            return result.get(0);
        }

        return null;
    }

    @Override
    public List<Stadium> findAll() {
        String sql = "SELECT * FROM Stadium";

        return jdbcTemplate.query(sql, new StadiumMapper());
    }

    @Override
    public void save(Stadium stadium) {
        String sql = "INSERT INTO Stadium VALUES(?, ?, ?, ?)";

        jdbcTemplate.update(sql,
                stadium.getId(),
                stadium.getCity(),
                stadium.getName(),
                stadium.getCountry());

    }

    @Override
    public void update(Stadium stadium) {
        String sql = "UPDATE Stadium SET " +
                "xd = ?";

        jdbcTemplate.update(sql,
                stadium.getId(),
                stadium.getCity(),
                stadium.getName(),
                stadium.getCountry());
    }

    @Override
    public void delete(String id) {
        String sql = "DELETE FROM Stadium WHERE id = ?";

        jdbcTemplate.update(sql, id);
    }
}
