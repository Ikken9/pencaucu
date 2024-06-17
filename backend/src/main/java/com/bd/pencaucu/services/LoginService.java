package com.bd.pencaucu.services;

import com.bd.pencaucu.models.Login;
import com.bd.pencaucu.persistance.interfaces.LoginDao;
import lombok.RequiredArgsConstructor;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class LoginService implements UserDetailsService {

    private final LoginDao loginDao;

    @Override
    public UserDetails loadUserByUsername(String username) throws UsernameNotFoundException {
        return loginDao.findById(username);
    }

    public void saveLoginUser(Login login) { loginDao.save(login); }

    public void updateUserLogin(Login login) { loginDao.update(login); }

    public void deleteUserLogin(String id) { loginDao.delete(id); }

}
