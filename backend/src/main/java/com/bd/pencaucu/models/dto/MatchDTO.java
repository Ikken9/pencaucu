package com.bd.pencaucu.models.dto;

import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.Setter;
import java.io.Serializable;
import java.sql.Date;

@Getter
@Setter
@RequiredArgsConstructor
public class MatchDTO implements Serializable {
    private int id;
    private Date date;
    private String knockoutStage;
    private String stadiumName;
    private String teamName;
    private String facedTeamName;
    private Integer teamScore;
    private Integer facedTeamScore;
    private String teamPictureUrl;
    private String facedTeamPictureUrl;
}
