package com.bd.pencaucu.domain.models;

import lombok.Getter;
import lombok.Setter;
import java.sql.Date;

@Getter
@Setter
public class Match {
    private Team firstTeam;
    private Team secondTeam;
    private int firstTeamScore;
    private int secondTeamScore;
    private Date date;
}
