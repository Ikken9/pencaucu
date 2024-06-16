package com.bd.pencaucu.services;

import com.bd.pencaucu.models.Stadium;
import com.bd.pencaucu.persistance.interfaces.StadiumDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class StadiumService {

    private final StadiumDao stadiumDao;

    public List<Stadium> getAllStadiums() {
        return stadiumDao.findAll();
    }

    public Stadium getStadiumById(String id) {
        return stadiumDao.findById(id);
    }

    public void createStadium(Stadium stadium) {
        stadiumDao.save(stadium);
    }

    public void updateStadium (Stadium stadium) {
        stadiumDao.update(stadium);
    }

    public void deleteStadium(String id) {
        stadiumDao.delete(id);
    }
}
