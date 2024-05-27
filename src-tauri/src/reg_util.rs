use std::io::{Error, ErrorKind};
use std::{collections::HashMap, io};
use winreg::transaction::Transaction;
use winreg::{RegKey, HKEY};

pub(crate) fn get_reg_value(h_key: HKEY, sub_key: &str, value_name: &str) -> io::Result<String> {
    let def_h_key = RegKey::predef(h_key);
    let key = def_h_key.open_subkey(sub_key)?;
    key.get_value::<String, _>(value_name)
}

pub(crate) fn set_value_with_transaction(
    h_key: HKEY,
    reg_value: RegValue,
    transaction: &Transaction,
) -> io::Result<()> {
    let (reg_key, _) =
        RegKey::predef(h_key).create_subkey_transacted(reg_value.sub_key, transaction)?;
    for (name, value) in reg_value.values {
        reg_key.set_value(name, &value)?;
    }
    Ok(())
}

pub struct RegValue {
    pub sub_key: String,
    pub values: HashMap<String, String>,
}

pub(crate) fn set_values(h_key: HKEY, reg_values: Vec<RegValue>) -> io::Result<()> {
    let transaction = Transaction::new()?;
    for reg_value in reg_values {
        if let Err(e) = set_value_with_transaction(h_key, reg_value, &transaction) {
            transaction.rollback()?;
            return Err(e);
        }
    }
    transaction.commit()
}

pub(crate) fn delete_sub_key(h_key: HKEY, sub_key: &str) -> io::Result<()> {
    if sub_key.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "地址为空"));
    }
    let key = RegKey::predef(h_key);
    key.delete_subkey_all(sub_key)
}
