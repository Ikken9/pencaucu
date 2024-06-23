package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.mappers.models.dto.ProfileDTOMapper;
import com.bd.pencaucu.models.dto.ProfileDTO;
import com.bd.pencaucu.persistance.interfaces.ProfileDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@RequiredArgsConstructor
public class ProfileDaoImpl implements ProfileDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public ProfileDTO getProfileByEmail(String email) {
        String sql =    "SELECT profile_picture, u.username FROM Players p " +
                            "INNER JOIN Users u ON u.email = p.player_email " +
                        "WHERE player_email = ?";
        List<ProfileDTO> profiles = jdbcTemplate.query(sql, new ProfileDTOMapper(), email);

        if (!profiles.isEmpty()) {
            return profiles.get(0);
        }

        String PROFILE_NOT_FOUND_MSG = "Profile with email %s not found";
        throw new UsernameNotFoundException(String.format(PROFILE_NOT_FOUND_MSG, email));
    }
}
