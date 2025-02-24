use std::collections::HashMap;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use arangors::{client::reqwest::ReqwestClient, Database};
use serde_json::Value;
use crate::model::student::Student;
use std::sync::Arc;

pub async fn add(
    State(db): State<Arc<Database<ReqwestClient>>>,
    Json(student): Json<Student>
) -> (StatusCode, Json<Student>) {
    let new_student = Student::new(student.name, student.age, student.subject,student.id);
    let query = "INSERT @student INTO students RETURN NEW";
    let mut bind_vars = HashMap::new();
    bind_vars.insert("student", serde_json::to_value(&new_student).unwrap());

    let inserted: Vec<Student> = db
        .aql_bind_vars(query, bind_vars)
        .await
        .expect("OOPS QUERY");

    if let Some(student) = inserted.first() {
        return (StatusCode::CREATED, Json(student.clone()));
    }

    (StatusCode::INTERNAL_SERVER_ERROR, Json(new_student))
}

pub async fn getall(State(db): State<Arc<Database<ReqwestClient>>>) -> Json<Vec<Student>> {
    let query = "FOR doc IN students return doc";
    let students: Vec<Student> = db.aql_str(query).await.expect("Error fetching students");
    Json(students)
}

pub async fn get_student(
    State(db): State<Arc<Database<ReqwestClient>>>,
    Path(id): Path<String>
) -> (StatusCode, Json<Vec<Student>>) {
    let query = "FOR doc in students FILTER doc._key== @id RETURN doc";
    let mut bind_vars = HashMap::new();
    bind_vars.insert("id", serde_json::to_value(&id).unwrap());

    let students: Vec<Student> = db
        .aql_bind_vars(query, bind_vars)
        .await
        .expect("Error fetching student");

    if students.is_empty() {
        return (StatusCode::NOT_FOUND, Json(Vec::new()));
    }

    (StatusCode::OK, Json(students))
}

pub async fn del_student(
    State(db): State<Arc<Database<ReqwestClient>>>,
    Path(id): Path<String>
) -> (StatusCode, Json<String>) {
    let query = "REMOVE { _key: @id } IN students";
    let mut bind_vars = HashMap::new();
    bind_vars.insert("id", Value::String(id));

    let result = db.aql_bind_vars::<serde_json::Value>(query, bind_vars).await;

    match result {
        Ok(_) => (StatusCode::OK, Json("Student deleted successfully".to_string())),
        Err(_) => (StatusCode::NOT_FOUND, Json("Student not found".to_string())),
    }
}
