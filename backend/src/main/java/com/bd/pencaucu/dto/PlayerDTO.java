package com.bd.pencaucu.dto;

import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.Setter;

import java.io.Serializable;

@Getter
@Setter
@RequiredArgsConstructor
public class PlayerDTO implements Serializable {
    private String name;
    private String lastName;
    private int points;
}
