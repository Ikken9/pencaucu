package com.bd.pencaucu.domain.models;

import lombok.Getter;
import lombok.Setter;
import java.sql.Blob;

@Getter
@Setter
public class Team {
    private String name;
    private Blob picture;
}
