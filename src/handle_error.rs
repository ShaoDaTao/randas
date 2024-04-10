use thiserror::Error;

pub type MyResult<T> = std::result::Result<T, Err_Handler>;

#[allow(non_camel_case_types)]
#[derive(Debug, Error)]
pub enum Err_Handler{
    #[error("From: `{from}`, reason: `{reason}`")]
    PARSE_ERR{ from: String, reason: String},
    #[error("From: `{from}`, reason: `{reason}`")]
    Not_Matched_Err{ from: String, reason: String},
    #[error("From: `{from}`, reason: `{reason}`")]
    Index_Out_of_Bound { from: String, reason: String},
    #[error("From: `{from}`, reason: `{reason}`")]
    File_Not_Found { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    Sheet_Not_Found { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    DataSeries_Length_Should_Be_Same { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    Write_To_File_Failed { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    File_Close_Failed { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    Type_Error { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    Title_Error { from: String, reason: String},
    #[error("From: `{from}`, reason: `{reason}`")]
    Different_Type_Cannot_Add { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    Different_Type_Cannot_Sub { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    Different_Type_Cannot_Multi { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    Different_Type_Cannot_Divide { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    Different_Type_Cannot_Compare { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    Err_Or_Null_Invalid_Convertion { from: String, reason: String },
    #[error("From: `{from}`, reason: `{reason}`")]
    Null_Result { from: String, reason: String },
    
}