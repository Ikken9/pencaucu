package com.bd.pencaucu.models;

import lombok.*;

@Getter
@Setter
@AllArgsConstructor
@NoArgsConstructor
public class User {

    private String email;
    private String username;
    private String password;
    private String career;
    private String profilePictureUrl;
    private String champion;
    private String subchampion;

}
