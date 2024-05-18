package com.bd.pencaucu.domain.models;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import java.sql.Blob;

@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
public class Team {
    private String name;
    private Blob picture;
}
