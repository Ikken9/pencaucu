package com.bd.pencaucu.models;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Getter
@Setter
@AllArgsConstructor
@NoArgsConstructor
public class Rank {
    private String playerEmail;
    private String teamName;
    private int finalPosition;
}
