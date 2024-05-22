FROM maven:3.9.6 AS build
WORKDIR /build
COPY . .
RUN mvn clean package

FROM openjdk:17
WORKDIR /app
COPY --from=build /build/target/pencaucu.jar app.jar
EXPOSE 8080
CMD ["java", "-jar", "app.jar"]