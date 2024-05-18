package com.bd.pencaucu.domain.models;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import java.sql.Date;

@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
public class Match {
    private Team firstTeam;
    private Team secondTeam;
    private int firstTeamScore;
    private int secondTeamScore;
    private Date date;
}
