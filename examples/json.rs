use allow_me::{Decision, PolicyBuilder, Request, Result};

fn main() -> Result<()> {
    let json = r#"{
        "statements": [
            {
                "effect": "allow",
                "identities": [
                    "actor_a"
                ],
                "operations": [
                    "write"
                ],
                "resources": [
                    "resource_1"
                ]
            }
        ]
    }"#;

    // Construct the policy.
    let policy = PolicyBuilder::from_json(json).build()?;

    // Prepare request (e.g. from user input).
    let request = Request::new("actor_a", "write", "resource_1")?;

    // Evaluate the request.
    match policy.evaluate(&request)? {
        Decision::Allowed => println!("Allowed"),
        Decision::Denied => {
            panic!("Denied!")
        }
    };

    Ok(())
}
