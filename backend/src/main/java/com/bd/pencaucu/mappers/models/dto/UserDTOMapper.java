package com.bd.pencaucu.mappers.models.dto;

import com.bd.pencaucu.models.dto.UserDTO;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class UserDTOMapper implements RowMapper<UserDTO> {

    @Override
    public UserDTO mapRow(ResultSet rs, int rowNum) throws SQLException {
        UserDTO userDTO = new UserDTO();
        userDTO.setUsername(rs.getString("username"));

        return userDTO;
    }
}
