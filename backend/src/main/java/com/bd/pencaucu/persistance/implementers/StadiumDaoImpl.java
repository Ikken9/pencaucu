package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.Stadium;
import com.bd.pencaucu.mappers.models.StadiumMapper;
import com.bd.pencaucu.persistance.interfaces.StadiumDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@RequiredArgsConstructor
public class StadiumDaoImpl implements StadiumDao {
    
    private final JdbcTemplate jdbcTemplate;
    
    @Override
    public Stadium findById(String id) {
        String sql = "SELECT * FROM Stadiums WHERE id = ?";

        List<Stadium> result = jdbcTemplate.query(sql, new StadiumMapper(), id);

        if (!result.isEmpty()) {
            return result.get(0);
        }

        return null;
    }

    @Override
    public List<Stadium> findAll() {
        String sql = "SELECT * FROM Stadiums";

        return jdbcTemplate.query(sql, new StadiumMapper());
    }

    @Override
    public void save(Stadium stadium) {
        String sql = "INSERT INTO Stadiums VALUES(?, ?, ?, ?)";

        jdbcTemplate.update(sql,
                stadium.getId(),
                stadium.getCity(),
                stadium.getName(),
                stadium.getCountry());

    }

    @Override
    public void update(Stadium stadium) {
        String sql = "UPDATE Stadiums SET " +
                "xd = ?";

        jdbcTemplate.update(sql,
                stadium.getId(),
                stadium.getCity(),
                stadium.getName(),
                stadium.getCountry());
    }

    @Override
    public void delete(String id) {
        String sql = "DELETE FROM Stadiums WHERE id = ?";

        jdbcTemplate.update(sql, id);
    }
}
