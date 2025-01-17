use crate::{
    attribute::IppAttribute,
    operation::{CreateJob, GetPrinterAttributes, IppOperation, PrintJob, SendDocument},
    IppJobSource,
};

/// Builder to create IPP operations
pub struct IppOperationBuilder;

impl IppOperationBuilder {
    /// Create PrintJob operation
    ///
    /// * `source` - `IppJobSource`
    pub fn print_job<T>(source: T) -> PrintJobBuilder
    where
        IppJobSource: From<T>,
    {
        PrintJobBuilder::new(source.into())
    }

    /// Create GetPrinterAttributes operation
    pub fn get_printer_attributes() -> GetPrinterAttributesBuilder {
        GetPrinterAttributesBuilder::new()
    }

    /// Create CreateJob operation
    pub fn create_job() -> CreateJobBuilder {
        CreateJobBuilder::new()
    }

    /// Create SendDocument operation
    ///
    /// * `job_id` - job id returned by Create-Job operation <br/>
    /// * `source` - `IppJobSource` <br/>
    pub fn send_document<T>(job_id: i32, source: T) -> SendDocumentBuilder
    where
        IppJobSource: From<T>,
    {
        SendDocumentBuilder::new(job_id, source.into())
    }
}

/// Builder to create PrintJob operation
pub struct PrintJobBuilder {
    source: IppJobSource,
    user_name: Option<String>,
    job_title: Option<String>,
    attributes: Vec<IppAttribute>,
}

impl PrintJobBuilder {
    fn new(source: IppJobSource) -> PrintJobBuilder {
        PrintJobBuilder {
            source,
            user_name: None,
            job_title: None,
            attributes: Vec::new(),
        }
    }
    /// Specify requesting-user-name attribute
    pub fn user_name(mut self, user_name: &str) -> Self {
        self.user_name = Some(user_name.to_owned());
        self
    }

    /// Specify job-name attribute
    pub fn job_title(mut self, job_title: &str) -> Self {
        self.job_title = Some(job_title.to_owned());
        self
    }

    /// Specify custom job attribute
    pub fn attribute(mut self, attribute: IppAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Build operation
    pub fn build(self) -> impl IppOperation {
        let op = PrintJob::new(self.source, self.user_name.as_ref(), self.job_title.as_ref());
        self.attributes.into_iter().fold(op, |mut op, attr| {
            op.add_attribute(attr);
            op
        })
    }
}

/// Builder to create GetPrinterAttributes operation
pub struct GetPrinterAttributesBuilder {
    attributes: Vec<String>,
}

impl GetPrinterAttributesBuilder {
    fn new() -> GetPrinterAttributesBuilder {
        GetPrinterAttributesBuilder { attributes: Vec::new() }
    }

    /// Specify which attribute to retrieve from the printer. Can be repeated.
    pub fn attribute(mut self, attribute: &str) -> Self {
        self.attributes.push(attribute.to_owned());
        self
    }

    /// Specify which attributes to retrieve from the printer
    pub fn attributes<T>(mut self, attributes: &[T]) -> Self
    where
        T: AsRef<str>,
    {
        self.attributes
            .extend(attributes.iter().map(|s| s.as_ref().to_string()));
        self
    }

    /// Build operation
    pub fn build(self) -> impl IppOperation {
        GetPrinterAttributes::with_attributes(&self.attributes)
    }
}

/// Builder to create CreateJob operation
pub struct CreateJobBuilder {
    job_name: Option<String>,
    attributes: Vec<IppAttribute>,
}

impl CreateJobBuilder {
    fn new() -> CreateJobBuilder {
        CreateJobBuilder {
            job_name: None,
            attributes: Vec::new(),
        }
    }

    /// Specify job-name attribute
    pub fn job_name(mut self, job_name: &str) -> Self {
        self.job_name = Some(job_name.to_owned());
        self
    }

    /// Specify custom job attribute
    pub fn attribute(mut self, attribute: IppAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Build operation
    pub fn build(self) -> impl IppOperation {
        let op = CreateJob::new(self.job_name.as_ref());
        self.attributes.into_iter().fold(op, |mut op, attr| {
            op.add_attribute(attr);
            op
        })
    }
}

/// Builder to create SendDocument operation
pub struct SendDocumentBuilder {
    job_id: i32,
    source: IppJobSource,
    user_name: Option<String>,
    is_last: bool,
}

impl SendDocumentBuilder {
    fn new(job_id: i32, source: IppJobSource) -> SendDocumentBuilder {
        SendDocumentBuilder {
            job_id,
            source,
            user_name: None,
            is_last: true,
        }
    }

    /// Specify originating-user-name attribute
    pub fn user_name(mut self, user_name: &str) -> Self {
        self.user_name = Some(user_name.to_owned());
        self
    }

    /// Parameter which indicates whether this document is a last one
    pub fn last(mut self, last: bool) -> Self {
        self.is_last = last;
        self
    }

    /// Build operation
    pub fn build(self) -> impl IppOperation {
        SendDocument::new(self.job_id, self.source, self.user_name.as_ref(), self.is_last)
    }
}
