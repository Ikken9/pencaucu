package com.bd.pencaucu.exceptions;

public class ResourceAlreadyExistsException extends RuntimeException {
    public ResourceAlreadyExistsException() {
        super("The resource already exists");
    }
}
