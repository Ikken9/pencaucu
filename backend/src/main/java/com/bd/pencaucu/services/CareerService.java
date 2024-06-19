package com.bd.pencaucu.services;

import com.bd.pencaucu.models.Career;
import com.bd.pencaucu.persistance.interfaces.CareerDao;
import lombok.AllArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@AllArgsConstructor
public class CareerService {

    private final CareerDao careerDao;

    public List<Career> getAllCareers() { return careerDao.findAll(); }

    public void createCareer(Career career) { careerDao.save(career); }

    public void updateCareer(Career career) { careerDao.update(career); }

    public void deleteCareer(String id) { careerDao.delete(id); }
}
