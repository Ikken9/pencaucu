package com.bd.pencaucu.domain.models;

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
public class User implements UserDetails {

    private String email;
    private String name;
    private String password;

    private AdminService adminService;

    @Override
    public Collection<? extends GrantedAuthority> getAuthorities() {
        Admin adminUser = adminService.getAdminById(email);
        if (adminUser != null) {
            SimpleGrantedAuthority authority = new SimpleGrantedAuthority("ROLE_ADMIN");
            return Collections.singletonList(authority);
        }
        SimpleGrantedAuthority authority = new SimpleGrantedAuthority("ROLE_USER");
        return Collections.singletonList(authority);
    }

    @Override
    public String getUsername() {
        return email;
    }

    @Override
    public boolean isAccountNonExpired() {
        return true;
    }

    @Override
    public boolean isAccountNonLocked() {
        return true;
    }

    @Override
    public boolean isCredentialsNonExpired() {
        return true;
    }

    @Override
    public boolean isEnabled() {
        return true;
    }
}
