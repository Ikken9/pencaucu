package com.bd.pencaucu.mappers.models.dto;

import com.bd.pencaucu.models.dto.ProfileDTO;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class ProfileDTOMapper implements RowMapper<ProfileDTO> {
    @Override
    public ProfileDTO mapRow(ResultSet rs, int rowNum) throws SQLException {
        ProfileDTO profile = new ProfileDTO();
        profile.setUsername(rs.getString("username"));
        profile.setProfilePicture(rs.getString("profile_picture"));

        return profile;
    }
}
