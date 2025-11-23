fn main() {
    let mut admin_roles: Vec<&str> = Vec::new();
    admin_roles.push("Intern");
    admin_roles.push("Administrator");
    admin_roles.push("Senior Administrator");

    println!("Admin roles (Vec::new): {:?}", admin_roles);

    let lawyer_roles = vec![
    "Paralegal",
    "Junior Associate",
    "Associate",
    ];

    println!("Lawyer roles (vec![]): {:?}", lawyer_roles);

    let role = "Associate";

    if lawyer_roles.contains(&role) {
        println!("'{}' belongs to Lawyer roles!", role);
    }
}