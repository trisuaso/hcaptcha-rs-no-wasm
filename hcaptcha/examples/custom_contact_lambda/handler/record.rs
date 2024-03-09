use super::error::ContactError;
use super::send::ContactForm;
use tracing::instrument;

#[instrument(
    name = "Write record to database"
    skip(form)
    fields(email = %form.email)
)]
pub async fn write(form: &ContactForm) -> Result<(), ContactError> {
    // Write the contact form data to dynamodb
    Ok(())
}
