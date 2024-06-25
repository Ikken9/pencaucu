package com.bd.pencaucu.models.dto;

import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.Setter;
import lombok.ToString;

import java.io.Serial;
import java.io.Serializable;
import java.sql.Timestamp;

@Getter
@Setter
@RequiredArgsConstructor
@ToString
public class MatchDTO implements Serializable {
    @Serial
    private static final long serialVersionUID = 1L;

    private int id;
    private Timestamp date;
    private String knockoutStage;
    private String stadiumName;
    private String teamName;
    private String facedTeamName;
    private Integer teamScore;
    private Integer facedTeamScore;
    private String teamPictureUrl;
    private String facedTeamPictureUrl;
}
