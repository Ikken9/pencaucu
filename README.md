# Penca UCU

## Introducción:

Este proyecto pretende cumplir con lo requerido para el trabajo final obligatorio de la materia Bases de Datos II.

El sistema diseñado para la PencaUCU asociada a la Copa América 2024 en la Universidad Católica del Uruguay tiene como objetivo principal permitir la participación de alumnos mediante un proceso de registro que incluye la obligación de ingresar predicciones para el campeón y subcampeón del torneo al momento de inscribirse, datos que no podrán ser modificados posteriormente. Los usuarios podrán ingresar sus predicciones de resultados para cada partido antes de que transcurra una hora desde el inicio del mismo, con la flexibilidad de realizar modificaciones hasta dicho momento límite. Un usuario administrador, que no será un alumno, será responsable de cargar los resultados de los partidos jugados, asegurándose de que estos estén actualizados en tiempo real en la plataforma.

Los participantes tendrán acceso a una interfaz que les permitirá visualizar las predicciones que han registrado y los puntos obtenidos por cada una de ellas, una vez que se haya completado el partido correspondiente. Además, podrán consultar un ranking general que mostrará los resultados acumulados por todos los usuarios participantes, ordenados por la cantidad de puntos obtenidos.

El sistema estará respaldado por una base de datos relacional SQL, que gestionará de manera segura la información de los usuarios, sus predicciones, los resultados de los partidos y cualquier estadística relevante para evaluar el desempeño de los participantes. Se contempla la posibilidad de realizar análisis estadísticos futuros sobre el rendimiento de los usuarios, potencialmente considerando datos adicionales como la carrera que cursan, para proporcionar insights valiosos sobre los patrones de acierto según distintos perfiles.

En resumen, la aplicación estará orientada a ofrecer una experiencia intuitiva y funcional para los usuarios, facilitando el registro, la gestión de predicciones, la actualización de resultados y la consulta de estadísticas, todo ello asegurando que cumpla con los estándares de seguridad y escalabilidad necesarios para su implementación y posibles expansiones futuras.

## Tecnologías Utilizadas

- **Lenguajes de Programación:** Java 17 (backend), Rust 1.81.0-nightly (frontend)
- **Framework de Desarrollo Backend:** Spring Boot 3.3.1
- **RDBMS:** MySQL
- **Framework de Desarrollo Frontend:** Leptos
- **Contenerización:** Docker

## Requisitos

- **Docker Desktop**

## Instrucciones

1. Clone este repositorio en su máquina local: `git clone https://github.com/Ikken9/pencaucu.git`
    
2. Diríjase la carpeta la cual contenga al proyecto que acaba de clonar
    
3. Abra la aplicación **Docker Desktop** (o en su defecto inicialice el *Docker daemon*)
    
4. Ejecute en consola el siguiente comando para levantar la aplicación:
    - `docker compose up`
	
5. Puede acceder a la aplicación desde:
    - [http://localhost](http://localhost/)

## Importante

El script de inicialización de la Base de Datos MySQL se encuentra ubicado entro de la carpeta "backend" bajo el nombre de "**init.sql**"

## Integrantes

- Piero Saucedo
- Martín Caraballo
- Juan Martín Riccetto
