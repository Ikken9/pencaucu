package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.mappers.models.StageMapper;
import com.bd.pencaucu.models.Stage;
import com.bd.pencaucu.persistance.interfaces.StageDao;
import lombok.AllArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@Repository
@AllArgsConstructor
public class StageDaoImpl implements StageDao {

    JdbcTemplate jdbcTemplate;

    @Override
    public List<Stage> getAllStages() {
        String sql = "SELECT id, name FROM Knockout_Stage";

        return jdbcTemplate.query(sql, new StageMapper());
    }
}
