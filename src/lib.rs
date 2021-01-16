/// The paste namespace which contains
/// every method and struct to
/// `GET` and `POST` (send) a paste
/// to [pastemyst](https://paste.myst.rs).
///
/// ### [API Docs]
/// Here is the official PasteMyst API
/// documentation:
/// https://paste.myst.rs/api-docs/index
#[allow(dead_code, unused_variables)]
pub mod paste {
    use serde::Deserialize;
    use serde::Serialize;

    /// The PasteResult type provided
    /// by this library for ease. It
    /// has a return value and error.
    ///
    /// ## Examples
    /// ```rust
    /// use pastemyst::paste::PasteResult;
    ///
    /// fn main() -> PasteResult<()> {
    ///     Ok(())
    /// }
    /// ```
    pub type PasteResult<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
    /// An enum of PasteMyt language constants.
    //#[allow(non_camel_case_types)]
    pub mod language {
        pub const AUTODETECT: &str = "Autodetect";
        pub const PLAIN: &str = "Plain Text";
        pub const APL: &str = "APL";
        pub const PGP: &str = "PGP";
        pub const ASN1: &str = "ASN.1";
        pub const ASTERISK: &str = "Asterisk";
        pub const BRAINFUCK: &str = "Brainfuck";
        pub const CLANG: &str = "C";
        pub const C: &str = "C";
        pub const CPP: &str = "C++";
        pub const COBOL: &str = "Cobol";
        pub const CSHARP: &str = "C#";
        pub const CLOJURE: &str = "Clojure";
        pub const CLOJURE_SCRIPT: &str = "ClojureScript";
        pub const GSS: &str = "Closure Stylesheets (GSS)";
        pub const CMAKE: &str = "CMake";
        pub const COFFEE_SCRIPT: &str = "CoffeeScript";
        pub const LISP: &str = "Common Lisp";
        pub const CYPHER: &str = "Cypher";
        pub const CYTHON: &str = "Cython";
        pub const CRYSTAL: &str = "Crystal";
        pub const CSS: &str = "CSS";
        pub const CQL: &str = "CQL";
        pub const DLANG: &str = "D";
        pub const D: &str = "D";
        pub const DART: &str = "Dart";
        pub const DIFF: &str = "diff";
        pub const DJANGO: &str = "Django";
        pub const DOCKER: &str = "Dockerfile";
        pub const DTD: &str = "DTD";
        pub const DYLAN: &str = "Dylan";
        pub const EBNF: &str = "EBNF";
        pub const ECL: &str = "ECL";
        pub const EDN: &str = "edn";
        pub const EIFFEL: &str = "Eiffel";
        pub const ELM: &str = "Elm";
        pub const EJS: &str = "Embedded Javascript";
        pub const ERB: &str = "Embedded Ruby";
        pub const ERLANG: &str = "Erlang";
        pub const ESPER: &str = "Esper";
        pub const FACTOR: &str = "Factor";
        pub const FCL: &str = "FCL";
        pub const FORTH: &str = "Forth";
        pub const FORTRAN: &str = "Fortran";
        pub const FSHARP: &str = "F#";
        pub const GAS: &str = "Gas";
        pub const GHERKIN: &str = "Gherkin";
        pub const GFM: &str = "GitHub Flavored Markdown";
        pub const GITHUB_MARKDOWN: &str = "GitHub Flavored Markdown";
        pub const GO: &str = "Go";
        pub const GROOVY: &str = "Groovy";
        pub const HAML: &str = "HAML";
        pub const HASKELL: &str = "Haskell";
        pub const HASKELL_LITERATE: &str = "Haskell (Literate)";
        pub const HAXE: &str = "Haxe";
        pub const HXML: &str = "HXML";
        pub const ASP_NET: &str = "ASP.NET";
        pub const HTML: &str = "HTML";
        pub const HTTP: &str = "HTTP";
        pub const IDL: &str = "IDL";
        pub const PUG: &str = "Pug";
        pub const JAVA: &str = "Java";
        pub const JSP: &str = "Java Server Pages";
        pub const JAVASCRIPT: &str = "JavaScript";
        pub const JSON: &str = "JSON";
        pub const JSON_LD: &str = "JSON-LD";
        pub const JSX: &str = "JSX";
        pub const JINJA2: &str = "Jinja2";
        pub const JULIA: &str = "Julia";
        pub const KOTLIN: &str = "Kotlin";
        pub const LESS: &str = "LESS";
        pub const LIVESCRIPT: &str = "LiveScript";
        pub const LUA: &str = "Lua";
        pub const MARKDOWN: &str = "Markdown";
        pub const MIRC: &str = "mIRC";
        pub const MARIA_DB: &str = "MariaDB SQL";
        pub const MATHEMATICA: &str = "Mathematica";
        pub const MODELICA: &str = "Modelica";
        pub const MUMPS: &str = "MUMPS";
        pub const MS_SQL: &str = "MS SQL";
        pub const MBOX: &str = "mbox";
        pub const MYSQL: &str = "MySQL";
        pub const NGINX: &str = "Nginx";
        pub const NSIS: &str = "NSIS";
        pub const NTRIPLES: &str = "NTriples";
        pub const OBJ_C: &str = "Objective-C";
        pub const OCAML: &str = "OCaml";
        pub const OCTAVE: &str = "Octave";
        pub const OZ: &str = "Oz";
        pub const PASCAL: &str = "Pascal";
        pub const PEG_JS: &str = "PEG.js";
        pub const PERL: &str = "Perl";
        pub const PHP: &str = "PHP";
        pub const PIG: &str = "Pig";
        pub const PLSQL: &str = "PLSQL";
        pub const POWERSHELL: &str = "PowerShell";
        pub const INI: &str = "Properties files";
        pub const PROTOBUF: &str = "ProtoBuf";
        pub const PYTHON: &str = "Python";
        pub const PUPPET: &str = "Puppet";
        pub const QLANG: &str = "Q";
        pub const RSCRIPT: &str = "R";
        pub const RST: &str = "reStructuredText";
        pub const RPM_CHANGES: &str = "RPM Changes";
        pub const RPM_SPEC: &str = "RPM Spec";
        pub const RUBY: &str = "Ruby";
        pub const RUST: &str = "Rust";
        pub const SAS: &str = "SAS";
        pub const SASS: &str = "Sass";
        pub const SCALA: &str = "Scala";
        pub const SCHEME: &str = "Scheme";
        pub const SCSS: &str = "SCSS";
        pub const SHELL: &str = "Shell";
        pub const SIEVE: &str = "Sieve";
        pub const SLIM: &str = "Slim";
        pub const SMALLTALK: &str = "Smalltalk";
        pub const SMARTY: &str = "Smarty";
        pub const SOLR: &str = "Solr";
        pub const SML: &str = "SML";
        pub const SOY: &str = "Soy";
        pub const SPARQL: &str = "SPARQL";
        pub const SPREADSHEET: &str = "Spreadsheet";
        pub const SQL: &str = "SQL";
        pub const SQLITE: &str = "SQLite";
        pub const SQUIRREL: &str = "Squirrel";
        pub const STYLUS: &str = "Stylus";
        pub const SWIFT: &str = "SWIFT";
        pub const STEX: &str = "sTeX";
        pub const LATEX: &str = "LaTeX";
        pub const SYSTEM_VERILOG: &str = "SystemVerilog";
        pub const TCL: &str = "Tcl";
        pub const TEXTILE: &str = "Textile";
        pub const TIDDLYWIKI: &str = "TiddlyWiki";
        pub const TIKI_WIKI: &str = "Tiki Wiki";
        pub const TOML: &str = "TOML";
        pub const TORNADO: &str = "Tornado";
        pub const TROFF: &str = "troff";
        pub const TTCN: &str = "TTCN";
        pub const TTCN_CFG: &str = "TTCN_CFG";
        pub const TURTLE: &str = "Turtle";
        pub const TYPESCRIPT: &str = "TypeScript";
        pub const TYPESCRIPT_JSX: &str = "TypeScript-JSX";
        pub const TWIG: &str = "Twig";
        pub const WEB_IDL: &str = "Web IDL";
        pub const VB_NET: &str = "VB.NET";
        pub const VBSCRIPT: &str = "VBScript";
        pub const VELOCITY: &str = "Velocity";
        pub const VERILOG: &str = "Verilog";
        pub const VHDL: &str = "VHDL";
        pub const VUE: &str = "Vue.js Component";
        pub const XML: &str = "XML";
        pub const XQUERY: &str = "XQuery";
        pub const YACAS: &str = "Yacas";
        pub const YAML: &str = "YAML";
        pub const Z80: &str = "Z80";
        pub const MSCGEN: &str = "mscgen";
        pub const XU: &str = "xu";
        pub const MSGENNY: &str = "msgenny";
    }

    const ENDPOINT: &str = "https://paste.myst.rs/";
    const BASE_ENDPOINT: &str = "https://paste.myst.rs/api/v2/";
    /// This endpoint is temporarily here due to a bug in pastemyst
    /// which does not allow the paste to be end when the last
    /// slash is present.
    const SEND_ENDPOINT: &str = "https://paste.myst.rs/api/v2/paste";
    const PASTE_ENDPOINT: &str = "https://paste.myst.rs/api/v2/paste/";

    /// Gets a paste's data in json format
    /// from [pastemyst](https://paste.myst.rs)
    /// synchronously. It returns a `Result`
    /// with a `PasteObject` and error.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::get_paste;
    /// use pastemyst::paste::PasteResult;
    ///
    /// fn main() -> PasteResult<()> {
    ///     let foo = get_paste("hipfqanx");
    ///     println!("{:?}", foo.title);
    ///     Ok(())
    /// }
    /// ```
    ///
    pub fn get_paste(id: &str) -> Result<PasteObject, reqwest::Error> {
        let info: PasteObject = reqwest::blocking::get(&parse_url(id))?.json()?;
        Ok(info)
    }

    /// Gets a paste's data in json format
    /// from [pastemyst](https://paste.myst.rs)
    /// asynchronously. It returns a `Result`
    /// with a `PasteObject` and error.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::get_paste_async;
    /// use pastemyst::paste::PasteResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> PasteResult<()> {
    ///     let foo = get_paste_async("hipfqanx").await?;
    ///     println!("{:?}", foo._id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_paste_async(id: &str) -> Result<PasteObject, reqwest::Error> {
        let info: PasteObject = reqwest::get(&parse_url(id)).await?.json().await?;
        Ok(info)
    }

    /// Gets a private paste's data in json format
    /// from [pastemyst](https://paste.myst.rs)
    /// synchronously. It returns a `Result`
    /// with a `PasteObject` and error.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::get_private_paste;
    /// use pastemyst::paste::PasteResult;
    ///
    /// fn main() -> PasteResult<()> {
    ///     let foo = get_private_paste("pasteID", "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings");
    ///     println!("{:?}", foo._id);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_private_paste(id: &str, auth_token: &str) -> Result<PasteObject, reqwest::Error> {
        let info: PasteObject = reqwest::blocking::Client::builder()
            .build()?
            .get(&parse_url(id))
            .header("Authorization", auth_token)
            .send()?
            .json()?;
        Ok(info)
    }

    /// Gets a private paste's data in json format
    /// from [pastemyst](https://paste.myst.rs)
    /// asynchronously. It returns a `Result`
    /// with a `PasteObject` and error.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::get_private_paste_async;
    /// use pastemyst::paste::PasteResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let foo = get_private_paste_async("pasteID", "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings").await?;
    ///     println!("{}", paste.isPrivate);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_private_paste_async(
        id: &str,
        auth_token: &str,
    ) -> Result<PasteObject, reqwest::Error> {
        let info: PasteObject = reqwest::Client::builder()
            .build()?
            .get(&parse_url(&id))
            .header("Authorization", auth_token)
            .send()
            .await?
            .json()
            .await?;
        Ok(info)
    }

    /// Uses the `CreateObject` struct as a parameter for paste
    /// data to be constructed into json format and sent to
    /// [pastemyst](https://paste.myst.rs) in a synchronous manner.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::PastyObject;
    /// use pastemyst::paste::*;
    ///
    /// fn main() -> PasteResult<()> {
    ///     let pasties: Vec<PastyObject> = vec![
    ///             PastyObject {
    ///             _id: None,
    ///             language: Some(String::from("autodetect")),
    ///             title: Some(String::from("Pasty1")),
    ///             code: Some(String::from("Code")),
    ///         },
    ///         PastyObject {
    ///             _id: None,
    ///             language: Some(String::from("autodetect")),
    ///             title: Some(String::from("Pasty2")),
    ///             code: Some(String::from("Code")),
    ///         },
    ///     ];
    ///     let data: CreateObject = CreateObject {
    ///         title: String::from("[crates.io/crates/pastemyst] This is a title"),
    ///         expiresIn: String::from("1d"),
    ///         isPrivate: false,
    ///         isPublic: false,
    ///         tags: String::from(""),
    ///         pasties: pasties,
    ///     };
    ///     let paste = create_paste(data)?;
    ///     println!("{:#?}", paste._id);
    ///     Ok(())
    /// }
    /// ```
    pub fn create_paste(contents: CreateObject) -> Result<PasteObject, reqwest::Error> {
        let content_type = reqwest::header::HeaderValue::from_static("application/json");
        let result = reqwest::blocking::Client::builder()
            .build()?
            .post(SEND_ENDPOINT)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .body(serde_json::to_string(&contents).unwrap())
            .send()
            .unwrap();
        Ok(result.json()?)
    }

    /// Uses the `CreateObject` struct as a parameter for paste
    /// data to be constructed into json format and sent to
    /// [pastemyst](https://paste.myst.rs) in an asynchronous manner.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::PastyObject;
    /// use pastemyst::paste::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> PasteResult<()> {
    ///     let pasties: Vec<PastyObject> = vec![
    ///             PastyObject {
    ///             _id: None,
    ///             language: Some(String::from("autodetect")),
    ///             title: Some(String::from("Pasty1")),
    ///             code: Some(String::from("Code")),
    ///         },
    ///         PastyObject {
    ///             _id: None,
    ///             language: Some(String::from("autodetect")),
    ///             title: Some(String::from("Pasty2")),
    ///             code: Some(String::from("Code")),
    ///         },
    ///     ];
    ///     let data: CreateObject = CreateObject {
    ///         title: String::from("[crates.io/crates/pastemyst] This is a title"),
    ///         expiresIn: String::from("1d"),
    ///         isPrivate: false,
    ///         isPublic: false,
    ///         tags: String::from(""),
    ///         pasties: pasties,
    ///     };
    ///     let paste = paste::create_paste_async(data).await?;
    ///     println!("{:?}", paste._id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_paste_async(contents: CreateObject) -> Result<PasteObject, reqwest::Error> {
        let content_type = reqwest::header::HeaderValue::from_static("application/json");
        let result = reqwest::Client::builder()
            .build()?
            .post(SEND_ENDPOINT)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .body(serde_json::to_string(&contents).unwrap())
            .send()
            .await?;
        Ok(result.json().await?)
    }

    /// Uses the `CreateObject` and `&str` (`auth_token`) to
    /// send a paste to [pastemyst](https://paste.myst.rs)
    /// held under your account which you can configure
    /// to be private/public or not. You also get the
    /// authority to delete that paste. This is a 
    /// synchronous method.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::create_private_paste;
    /// use pastemyst::paste::get_paste;
    /// use pastemyst::paste::PasteResult;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let contents = get_paste(hipfqanx);
    ///     let paste = create_private_paste(contents, "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings").await?;
    ///     println!("{}", paste.isPrivate);
    ///     Ok(())
    /// }
    /// ```
    pub fn create_private_paste(
        contents: CreateObject,
        auth_token: &str,
    ) -> Result<PasteObject, reqwest::Error> {
        let content_type = reqwest::header::HeaderValue::from_static("application/json");
        let result = reqwest::blocking::Client::builder()
            .build()?
            .post(SEND_ENDPOINT)
            .header("Authorization", auth_token)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .body(serde_json::to_string(&contents).unwrap())
            .send()?;
        Ok(result.json()?)
    }

    /// Uses the `CreateObject` struct and a `&str` authorization
    /// key as a parameter which you can get from user settings
    /// on [pastemyst](https://paste.myst.rs/user/settings).
    /// This data is constructed into json format and sent to
    /// [pastemyst](https://paste.myst.rs)
    /// in an asynchronous manner. The paste is send under
    /// the ownership of the account the auth key belongs to.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use pastemyst::paste::*;
    /// 
    /// fn main() -> Result<(), reqwest::Error> /*PasteResult<()>*/ {
    ///     let pasties: Vec<PastyObject> = vec![
    ///         PastyObject {
    ///             _id: None,
    ///             language: Some(String::from("autodetect")),
    ///             title: Some(String::from("A pasty title")),
    ///             code: Some(String::from("fn main() { println!(\"Hello World!\"); }")),
    ///         },
    ///         PastyObject {
    ///             _id: None,
    ///             title: Some(String::from("Another pasty title")),
    ///             language: Some(String::from("autodetect")),
    ///             code: Some(String::from(
    ///                 "#include \"stdio.h\"\n\nint main() {\n\tprintf(\"Hello World!\");\n}",
    ///             )),
    ///         },
    ///     ];
    ///     let data: CreateObject = CreateObject {
    ///         title: String::from("[crates.io/crates/pastemyst] This is a title"),
    ///         expiresIn: String::from("1d"),
    ///         isPrivate: false,
    ///         isPublic: false,
    ///         tags: String::from(""),
    ///         pasties,
    ///     };
    ///     let paste = create_private_paste(
    ///         data,
    ///         "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings",
    ///     )?;
    ///     println!("{:#?}", paste.ownerId);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_private_paste_async(
        contents: CreateObject,
        auth_token: &str,
    ) -> Result<PasteObject, reqwest::Error> {
        let content_type = reqwest::header::HeaderValue::from_static("application/json");
        let result = reqwest::Client::builder()
            .build()?
            .post(SEND_ENDPOINT)
            .header("Authorization", auth_token)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .body(serde_json::to_string(&contents).unwrap())
            .send()
            .await?;
        Ok(result.json().await?)
    }

    /// You can only delete pastes on your account, which
    /// means you must also provide the authorization key.
    /// This action is irreversible can the paste cannot
    /// be restored in any way.
    ///
    /// This method returns an unsigned 16 bit integer
    /// which is a status code recieved by the PasteMyst
    /// server. If a paste deletes successfully, you
    /// should recieve a status code of `200`. For
    /// a list of all the web status codes, refer to:
    /// https://en.wikipedia.org/wiki/List_of_HTTP_status_codes
    ///
    /// ### API Docs
    /// The relevent link to the API Documentation
    /// is: https://paste.myst.rs/api-docs/paste
    ///
    /// ```rust
    /// use pastemyst::paste::*;
    ///
    /// fn main() -> PasteResult<()> {
    ///     let paste_del_result = delete_paste(
    ///         "PasteID",
    ///         "Your PasteMyst Token. Get it from: https://paste.myst.rs/user/settings",
    ///     )?;
    ///     if (paste_del_result == 200) { println!("Paste has been deleted successfully."); }
    ///     else { println!("Something went wrong and we recieved a status code of {}", paste_del_result); }
    ///     Ok(())
    /// }
    /// ```
    pub fn delete_paste(id: &str, auth_token: &str) -> Result<u16, reqwest::Error> {
        let result = reqwest::blocking::Client::builder()
            .build()?
            .delete(&parse_url(&id))
            .header("Authorization", auth_token)
            .send()?;
        Ok(result.status().as_u16())
    }

    /// Parses the url by combining
    /// the `PASTE_ENDPOINT` with a
    /// provided id.
    fn parse_url(id: &str) -> String { return PASTE_ENDPOINT.to_owned() + &id }

    /// The paste object recieved when
    /// getting a paste. It contains
    /// both the `PastyObject` and
    /// `EditObject` in an array.
    ///
    /// ### API Docs
    /// The relevent link to the API documentation
    /// is: https://paste.myst.rs/api-docs/objects
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let _foo: PasteObject = get_paste("hipfqanx");
    /// ```
    #[derive(Deserialize)]
    #[allow(non_snake_case, dead_code)]
    pub struct PasteObject {
        /// Id of the paste.
        pub _id: String,
        /// Id of the owner, if it doesn't
        ///  have an owner it's set to "".
        pub ownerId: String,
        /// Title of the paste.
        pub title: String,
        /// Unix time of when
        /// the paste is created.
        pub createdAt: u64,
        /// When the paste will expire,
        /// possible values are
        /// `never`, `1h`, `2h`, `10h`,
        /// `1d`, `2d`, `1w`, `1m`, `1y`.
        pub expiresIn: String,
        /// When the paste will be deleted, if
        /// it has no expiry time it's set to 0.
        pub deletesAt: u64,
        /// Number of stars the paste received.
        pub stars: u64,
        /// If it's private it's only
        /// accessible by the owner.
        pub isPrivate: bool,
        /// Is it displayed on the
        /// owner's public profile.
        pub isPublic: bool,
        /// List of tags.
        pub tags: Vec<String>,
        /// List of pasties/files in
        /// the paste, can't be empty.
        pub pasties: Vec<PastyObject>,
        /// List of edits.
        pub edits: Vec<EditObject>,
    }

    /// Information about a specific pasty in a paste.
    ///
    /// All fields except `language` are optional but due
    /// to Rust's nature, so you must provide them. The
    /// _id field should always be set to `None` though
    /// if it's not, it is ignored by PasteMyst's API.
    /// 
    /// The design choice of the language field not being
    /// optional was because auto detect isn't perfect
    /// and you generally should not rely on it especially
    /// with close bonded languages like C++ and C# which is
    /// sometimes confused by the language detector. However,
    /// you do not need to and can change set it to auto detect.
    ///
    /// ### API Docs
    /// The relevent link to the API documentation
    /// is: https://paste.myst.rs/api-docs/objects
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let pasty: PastyObject = PastyObject {
    ///     _id: None,
    ///     language: Some(String::from("autodetect")),
    ///     title: Some(String::from("This is a pasty title")),
    ///     code: Some(String::from("{\"This_Is\": \"JSON_Code\"}")),
    /// };
    /// ```
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case, dead_code)]
    pub struct PastyObject {
        /// Id of the pasty.
        pub _id: Option<String>,
        /// Language of the pasty.
        pub language: Option<String>,
        /// title of the pasty.
        pub title: Option<String>,
        /// contents of the pasty.
        pub code: Option<String>,
    }

    /// Infomation about edits in a pasty in a paste.
    ///
    /// ### API Docs
    /// The relevent link to the API documentation
    /// is: https://paste.myst.rs/api-docs/objects
    ///
    /// ## Examples
    ///
    /// ```rust
    /// // Get paste from pastemyst
    /// let edits: EditObject = paste.edits[0];
    /// ```
    #[derive(Deserialize)]
    #[allow(non_snake_case, dead_code)]
    pub struct EditObject {
        /// Unique id of the edit.
        pub _id: String,
        /// Id of the edit, multiple edits can
        /// share the same id showing that multiple
        /// fields were changed at the same time.
        pub editId: String,
        /// Type of edit, possible values are
        /// title(0), pastyTitle(1), pastyLanguage(2),
        /// pastyContent(3), pastyAdded(4), pastyRemoved(5).
        pub editType: i32,
        /// Various metadata used internally,
        /// biggest usecase is storing exactly which
        /// pasty was edited.
        pub metadata: Vec<String>,
        /// Actual paste edit, it stores old data
        /// before the edit as the current paste
        /// stores the new data
        pub edit: String,
        /// Unix time of when the edit was made
        pub editedAt: i32,
    }

    /// The structure object that holds
    /// the base to create a paste. This
    /// is then sent to pastemyst. All
    /// fields are optional *except* the
    /// `pasties` array which uses `PastyObject`.
    ///
    /// ### API Docs
    /// The relevent link to the API documentation
    /// is: https://paste.myst.rs/api-docs/objects
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let _data: CreateObject = CreateObject {
    ///     title: String::from("[crates.io/crates/pastemyst] This is a title"),
    ///     expiresIn: String::from("1d"),
    ///     isPrivate: false,
    ///     isPublic: false,
    ///     tags: String::from(""),
    ///     pasties: pasties,
    /// };
    /// ```
    #[derive(Serialize)]
    #[allow(non_snake_case, dead_code)]
    pub struct CreateObject {
        /// Title of the paste.
        pub title: String,
        /// When the paste will expire,
        /// possible values are never, 1h,
        /// 2h, 10h, 1d, 2d, 1w, 1m, 1y.
        pub expiresIn: String,
        /// If it"s private it"s only
        /// accessible by the owner.
        pub isPrivate: bool,
        /// Is it displayed on the
        /// owner's public profile.
        pub isPublic: bool,
        /// List of tags, comma separated.
        pub tags: String,
        /// List of pasties.
        pub pasties: Vec<PastyObject>,
    }

    /// The same as `CreateObject` except
    /// that it does not have the `expiresIn`
    /// field which has been removed for
    /// convenience. This may change in
    /// the future, but for the current
    /// moment, this shall remain.
    ///
    /// You can only edit pastes on your account,
    /// so you must provide the Authorization header.
    /// it returns a full paste object. To edit a paste
    /// you need to provide only the values you are
    /// editing in the JSON body.
    ///
    /// To edit a single pasty you will need to provide
    /// all of the original pasties changing the fields
    //// you want. it"s not possible to update a single
    /// pasty without providing all of the pasties.
    ///
    /// ### API Docs
    /// The relevent link to the API documentation
    /// is: https://paste.myst.rs/api-docs/paste#edit-a-paste
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let _data: CreateObject = CreateObject {
    ///     title: String::from("[crates.io/crates/pastemyst] This is a title"),
    ///     expiresIn: String::from("1d"),
    ///     isPrivate: false,
    ///     isPublic: false,
    ///     tags: String::from(""),
    ///     pasties: var_pasties,
    /// };
    /// ```
    #[derive(Serialize)]
    #[allow(non_snake_case, dead_code)]
    pub(crate) struct EditObject {
        /// Title of the paste.
        pub title: String,
        /// If it"s private it"s only
        /// accessible by the owner.
        pub isPrivate: bool,
        /// Is it displayed on the
        /// owner's public profile.
        pub isPublic: bool,
        /// List of tags, comma separated.
        pub tags: String,
        /// List of pasties.
        pub pasties: Vec<PastyObject>,
    }
}
