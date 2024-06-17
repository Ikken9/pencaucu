package com.bd.pencaucu.models;

import com.bd.pencaucu.services.AdminService;
import lombok.*;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.core.userdetails.UserDetails;
import java.util.Collection;
import java.util.Collections;

@Getter
@Setter
@AllArgsConstructor
@NoArgsConstructor
public class User {

    private String email;
    private String username;
    private String password;
    private String career;

}
