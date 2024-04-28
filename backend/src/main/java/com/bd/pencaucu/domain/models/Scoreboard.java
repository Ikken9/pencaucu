package com.bd.pencaucu.domain.models;

import lombok.Getter;
import lombok.Setter;
import java.util.HashMap;

@Getter
@Setter
public class Scoreboard {
    private HashMap<User, Integer> users;
    private User first;
    private User second;
}