package com.bd.pencaucu.models;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import java.sql.Timestamp;

@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
public class Match {
    private int id;
    private Timestamp date;
    private int knockoutStageId;
    private String stadiumId;
    private String teamName;
    private String facedTeamName;
    private String adminEmail;
}
