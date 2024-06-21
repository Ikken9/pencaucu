package com.bd.pencaucu.models;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Getter
@Setter
@AllArgsConstructor
@NoArgsConstructor
public class Bet {
    private String playerEmail;
    private int matchId;
    private int teamScore;
    private int facedTeamScore;
}
