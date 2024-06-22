package com.bd.pencaucu.services;

import com.bd.pencaucu.models.Stage;
import com.bd.pencaucu.persistance.interfaces.StageDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class StageService {

    private final StageDao stageDao;

    public List<Stage> getAllStages() { return stageDao.getAllStages(); }
}
