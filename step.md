





Updated Checklist:

    ✅ Set up a new Rust project.
    ✅ Add necessary dependencies to Cargo.toml.
    ✅ Configure SQLite using SQLx.
✅ Define the database schema for users and tasks in src/models.rs.
✅ Implement database connection and initialization in src/db.rs.
✅ Create initial migrations for the database schema in the migrations/ directory.
✅ Implement user registration in src/api/auth.rs.
✅ Implement user login and JWT generation in src/api/auth.rs.
✅ (Optional) Implement user logout in src/api/auth.rs.
✅ Create utility functions for password hashing in src/utils.rs.
✅ Update src/main.rs to include auth routes.
✅ Implement task creation in src/api/tasks.rs.
✅ Implement task retrieval (all and by ID) in src/api/tasks.rs.
✅ Implement task updating in src/api/tasks.rs.
✅ Implement task deletion in src/api/tasks.rs.
✅ Update src/main.rs to include task routes, protected by the auth middleware.
✅ Create Leptos components for authentication (registration and login) in src/app.rs.
✅ Implement state management for user authentication on the frontend.
✅ Create Leptos components for displaying and managing tasks in src/app.rs.
✅ Implement API calls from the Leptos frontend to the Axum backend for tasks.
✅ Handle frontend routing to navigate between authentication and task views
