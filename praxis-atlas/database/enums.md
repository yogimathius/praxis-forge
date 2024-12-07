# Database Enums

## Priority

```sql
CREATE TYPE priority AS ENUM (
    'High',
    'Medium',
    'Low'
);
```

## Status

```sql
CREATE TYPE status AS ENUM (
    'Not Started',
    'In Progress',
    'Completed',
    'Archived'
);
```

## Frequency

```sql
CREATE TYPE frequency AS ENUM (
    'Daily',
    'Weekly',
    'Monthly',
    'Custom'
);
```

## ActivityType

```sql
CREATE TYPE activity_type AS ENUM (
    'Task_Complete',
    'Goal_Progress',
    'Habit_Check',
    'System_Event'
);
```

## RecommendationType

```sql
CREATE TYPE recommendation_type AS ENUM (
    'Task_Suggestion',
    'Goal_Advice',
    'Habit_Formation',
    'Productivity_Tip'
);
```
