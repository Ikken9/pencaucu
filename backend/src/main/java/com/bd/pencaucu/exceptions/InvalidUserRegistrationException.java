package com.bd.pencaucu.exceptions;

public class InvalidUserRegistrationException extends RuntimeException {
    public InvalidUserRegistrationException() {
        super("Failed to register user.");
    }
}
