package com.bd.pencaucu.services;

import com.bd.pencaucu.models.Login;
import com.bd.pencaucu.persistance.interfaces.LoginDao;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class LoginService {

    private final LoginDao loginDao;

    public Login findLoginId(String id) { return loginDao.findById(id); }

    public void saveLoginUser(Login login) { loginDao.save(login); }

    public void updateUserLogin(Login login) { loginDao.update(login); }

    public void deleteUserLogin(String id) { loginDao.delete(id); }


}
