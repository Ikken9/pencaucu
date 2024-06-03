package com.bd.pencaucu.domain.models;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Getter
@Setter
@NoArgsConstructor
public class User {
    private String email;
    private String name;
    private String password;
    private boolean isAdmin;
    private int userPoints;
}
