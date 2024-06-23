package com.bd.pencaucu.services;

import com.bd.pencaucu.models.dto.ProfileDTO;
import com.bd.pencaucu.persistance.interfaces.ProfileDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class ProfileService {

    private final ProfileDao profileDao;

    public ProfileDTO getProfileByEmail(String email) { return profileDao.getProfileByEmail(email); }
}
