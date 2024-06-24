package com.bd.pencaucu.services;

import lombok.RequiredArgsConstructor;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.mail.SimpleMailMessage;
import org.springframework.mail.javamail.JavaMailSender;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Service;
import java.nio.charset.StandardCharsets;
import java.util.List;

@Service
@RequiredArgsConstructor
public class NotificationService {
    private static final Logger logger = LoggerFactory.getLogger(NotificationService.class);

    private final MatchService matchService;
    private final PlayerService playerService;
    private final JavaMailSender emailSender;

    @Value("${spring.mail.username}")
    private String fromEmail;

    public void sendEmail(String to, String subject, String text) {
        SimpleMailMessage message = new SimpleMailMessage();
        message.setFrom(fromEmail);
        message.setTo(to);
        message.setSubject(subject);
        message.setText(text);

        emailSender.send(message);
    }

    @Scheduled(cron = "0 0 0 * * *")
    public void notifyUsers() {
        String teamName = matchService.getFirstMatchOfTheDay().getTeamName();
        String facedTeamName = matchService.getFirstMatchOfTheDay().getFacedTeamName();

        String subject = "First match of the day";
        String text = "The first match of the day is about to begin!\n " + teamName + " VS " + facedTeamName;

        List<String> emails = playerService.getAllPlayersEmails();
        System.out.println(emails);

        if (emails.isEmpty()) {
            logger.info("No emails to send.");
            return;
        }

        for (String email : emails) {
            logger.info("Sending email to: {}", email);
            String parsedEmail = new String(email.trim().getBytes(StandardCharsets.UTF_8), StandardCharsets.UTF_8);
            sendEmail(parsedEmail, subject, text);
        }
    }
}
