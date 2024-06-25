package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.mappers.models.CareerMapper;
import com.bd.pencaucu.models.Career;
import com.bd.pencaucu.persistance.interfaces.CareerDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@RequiredArgsConstructor
public class CareerDaoImpl implements CareerDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public List<Career> findAll() {
        String sql = "SELECT career_name FROM Careers";

        return jdbcTemplate.query(sql, new CareerMapper());
    }

    @Override
    public void save(Career career) {
        String sql = "INSERT INTO Careers (career_name) VALUES (?)";
        jdbcTemplate.update(sql, career.getCareerName());
    }

    @Override
    public void update(Career career) {
        String sql = "UPDATE Careers SET career_name = ? WHERE career_name = ?";

        jdbcTemplate.update(sql, career.getCareerName(), career.getCareerName());
    }

    @Override
    public void delete(String id) {
        String sql = "DELETE FROM Careers WHERE career_name = ?";
        jdbcTemplate.update(sql, id);
    }
}
