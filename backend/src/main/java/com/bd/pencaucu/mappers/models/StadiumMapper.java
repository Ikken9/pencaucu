package com.bd.pencaucu.mappers.models;

import java.sql.ResultSet;
import java.sql.SQLException;

import com.bd.pencaucu.domain.models.Stadium;
import org.springframework.jdbc.core.RowMapper;

public class StadiumMapper implements RowMapper<Stadium>{
    @Override
    public Stadium mapRow(ResultSet rs, int rowNum) throws SQLException{
        Stadium stadium = new Stadium();
        String stadiumID = rs.getString("id");
        String name = rs.getString("name");
        String country = rs.getString("Country");
        String city = rs.getString("city");
        
        stadium.setId(stadiumID);
        stadium.setName(name);
        stadium.setCountry(country);
        stadium.setCity(city);
        return stadium;

    }
}
