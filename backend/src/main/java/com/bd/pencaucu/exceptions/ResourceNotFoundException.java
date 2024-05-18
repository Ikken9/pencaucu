package com.bd.pencaucu.exceptions;

public class ResourceNotFoundException extends RuntimeException {
    public ResourceNotFoundException() {
        super("No resource was found");
    }
}
