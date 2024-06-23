package com.bd.pencaucu.persistance.interfaces;

import com.bd.pencaucu.models.dto.ProfileDTO;

public interface ProfileDao {
    ProfileDTO getProfileByEmail(String email);
}
