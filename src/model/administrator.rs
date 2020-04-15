#[path = "../controller/authentication/hashing.rs"] mod hashing;

use rocket::{Request, Outcome};
use rocket::request::FromRequest;

use std::collections::HashMap;
use std::string::ToString;

use diesel::{ self, prelude::* };

use super::auth::*;
use hashing::*;
use crate::DbConnection;
use crate::database::Users;

/// The AdministratorCookie type is used to indicate a user has logged in as an administrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministratorCookie {
    pub userid: u32,
    pub username: String,
    pub display: Option<String>,
}

/// The AdministratorForm type is used to process a user attempting to login as an administrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministratorForm {
    pub username: String,
    pub password: String,
}

impl CookieId for AdministratorCookie {
    fn cookie_id<'a>() -> &'a str {
        "acid"
    }
}

impl CookieId for AdministratorForm {
    fn cookie_id<'a>() -> &'a str {
        "acid"
    }
}

impl AuthorizeCookie for AdministratorCookie {
    // type CookieType = AdministratorCookie;

    /// The store_cookie() method should contain code that
    /// converts the specified data structure into a string
    ///
    /// This is likely to be achieved using one of the serde
    /// serialization crates.  Personally I would use either
    /// serde_json or serde's messagepack implementation ( rmp-serde [rmps]).
    ///
    /// Json is portable and human readable.
    ///
    /// MsgPack is a binary format, and while not human readable is more
    /// compact and efficient.
    fn store_cookie(&self) -> String {
        // String::from("This is my cooky")
        // let ser = ::serde_json::to_string(self).expect("Could not serialize");
        ::serde_json::to_string(self).expect("Could not serialize")
    }


    /// The retrieve_cookie() method deserializes a string
    /// into a cookie data type.
    ///
    /// Again, serde is likely to be used here.
    /// Either the messagepack or json formats would work well here.
    ///
    /// Json is portable and human readable.
    ///
    /// MsgPack is a binary format, and while not human readable is more
    /// compact and efficient.
    #[allow(unused_variables)]
    // fn retrieve_cookie(string: String) -> Option<Self::CookieType> {
    fn retrieve_cookie(string: String) -> Option<Self> {
        let mut des_buf = string.clone();
        let des: Result<AdministratorCookie, _> = ::serde_json::from_str(&mut des_buf);
        if let Ok(cooky) = des {
            Some(cooky)
        } else {
            None
        }
        // Some(
        //     AdministratorCookie {
        //         userid: 66,
        //         username: "andrew".to_string(),
        //         display: Some("Andrew Prindle".to_string()),
        //     }
        // )
    }
}

impl AuthorizeForm for AdministratorForm {
    type CookieType = AdministratorCookie;

    /// Authenticate the credentials inside the login form
    fn authenticate(&self, conn: DbConnection) -> Result<Self::CookieType, AuthFail> {
        use crate::schema::*;

        let user_keys: Vec<Users> = users::table
            .filter(users::username.eq(&self.username.to_string()))
            .load(&conn.0)
            .expect("User not found");
        if user_keys.len() > 0 {
            let claim = Password { random_salt: (&user_keys[0].salt).parse().unwrap(), .. Password::default() };
            if claim.verify(&self.password, &user_keys[0].password) {
                Ok(
                    AdministratorCookie {
                        userid: user_keys[0].id as u32,
                        username: self.username.to_string(),
                        display: Some("Administrator".to_string())
                    }
                )
            } else {
                Err(
                    AuthFail::new(self.username.to_string(), "Incorrect username or password.".to_string())
                )
            }
        } else {
            Err(
                AuthFail::new(self.username.to_string(), "Incorrect username or password.".to_string())
            )
        }

        //change here to authenticate the user in the database
        // println!("Authenticating {} with password: {}", &self.username, &self.password);
        // if &self.username == "administrator" && &self.password != "" {
        //     Ok(
        //         AdministratorCookie {
        //             userid: 1,
        //             username: "administrator".to_string(),
        //             display: Some("Administrator".to_string()),
        //         }
        //     )
        // } else {
        //     Err(
        //         AuthFail::new(self.username.to_string(), "Incorrect username".to_string())
        //     )
        // }
    }

    /// Create a new login form instance
    fn new_form(user: &str, pass: &str, _extras: Option<HashMap<String, String>>) -> Self {
        AdministratorForm {
            username: user.to_string(),
            password: pass.to_string(),
        }
    }

}

impl<'a, 'r> FromRequest<'a, 'r> for AdministratorCookie {
    type Error = ();

    /// The from_request inside the file defining the custom data types
    /// enables the type to be checked directly in a route as a request guard
    ///
    /// This is not needed but highly recommended.  Otherwise you would need to use:
    ///
    /// `#[get("/protected")] fn admin_page(admin: AuthCont<AdministratorCookie>)`
    ///
    /// instead of:
    ///
    /// `#[get("/protected")] fn admin_page(admin: AdministratorCookie)`
    fn from_request(request: &'a Request<'r>) -> ::rocket::request::Outcome<AdministratorCookie,Self::Error>{
        let cid = AdministratorCookie::cookie_id();
        let mut cookies = request.cookies();

        match cookies.get_private(cid) {
            Some(cookie) => {
                if let Some(cookie_deserialized) = AdministratorCookie::retrieve_cookie(cookie.value().to_string()) {
                    Outcome::Success(
                        cookie_deserialized
                    )
                } else {
                    Outcome::Forward(())
                }
            },
            None => Outcome::Forward(())
        }
    }
}