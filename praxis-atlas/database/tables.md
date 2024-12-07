# Database Tables

## Users

| Column        | Type         | Constraints |
| ------------- | ------------ | ----------- |
| user_id       | id           | PK          |
| username      | varchar(50)  | unique      |
| email         | varchar(100) | unique      |
| password_hash | varchar(255) | not null    |
| created_at    | timestamp    | not null    |
| last_login    | timestamp    |             |

## Tasks

| Column      | Type      | Constraints |
| ----------- | --------- | ----------- |
| task_id     | id        | PK          |
| user_id     | id        | FK          |
| goal_id     | id        | FK, null    |
| title       | title     | not null    |
| description | text      |             |
| due_date    | date      |             |
| priority    | Priority  | not null    |
| status      | Status    | not null    |
| created_at  | timestamp | not null    |
| updated_at  | timestamp | not null    |

## Goals

| Column      | Type      | Constraints |
| ----------- | --------- | ----------- |
| goal_id     | id        | PK          |
| user_id     | id        | FK          |
| title       | title     | not null    |
| description | text      |             |
| target_date | date      |             |
| progress    | float     | default: 0  |
| status      | Status    | not null    |
| created_at  | timestamp | not null    |
| updated_at  | timestamp | not null    |

## Habits

| Column      | Type      | Constraints |
| ----------- | --------- | ----------- |
| habit_id    | id        | PK          |
| user_id     | id        | FK          |
| goal_id     | id        | FK, null    |
| title       | title     | not null    |
| description | text      |             |
| frequency   | Frequency | not null    |
| streak      | integer   | default: 0  |
| created_at  | timestamp | not null    |
| updated_at  | timestamp | not null    |

## ActivityLogs (stretch goal)

| Column        | Type         | Constraints |
| ------------- | ------------ | ----------- |
| log_id        | id           | PK          |
| user_id       | id           | FK          |
| habit_id      | id           | FK, null    |
| task_id       | id           | FK, null    |
| activity_type | ActivityType | not null    |
| description   | text         |             |
| timestamp     | timestamp    | not null    |

## AIRecommendations (stretch goal)

| Column              | Type               | Constraints |
| ------------------- | ------------------ | ----------- |
| recommendation_id   | id                 | PK          |
| user_id             | id                 | FK          |
| recommendation_type | RecommendationType | not null    |
| content             | text               | not null    |
| created_at          | timestamp          | not null    |
| applied_at          | timestamp          |             |
