pub mod errors;

use std::fmt;
use std::str;

#[derive(Debug)]
pub struct Message {
    prefix: Option<String>,
    command: String,
    params: Vec<String>,
}

#[derive(Debug)]
struct Syntax {
    prefix: Option<String>,
    command: String,
    params: Vec<String>,
}

// RFC 1459 4, 5.
#[allow(non_snake_case)]
#[derive(Debug)]
enum Command {
    // 4.1 Connection Registration.
    NICK,
    PASS,
    USER,
    SERVER,
    OPER,
    QUIT,
    SQUIT,

    // 4.2 Channel Operations.
    JOIN,
    PART,
    MODE,
    TOPIC,
    NAMES,
    LIST,
    INVITE,
    KICK,

    // 4.3 Server queries and commands.
    VERSION,
    STATS,
    LINKS,
    TIME,
    CONNECT,
    TRACE,
    ADMIN,
    INFO,

    // 4.4 Sending messages.
    PRIVMSG,
    NOTICE,

    // 4.5 User based queries.
    WHO,
    WHOIS,
    WHOWAS,

    // 4.6 Misc.
    KILL,
    PING,
    PONG,
    ERROR,

    // 5 Optionals.
    AWAY,
    REHASH,
    RESTART,
    SUMMON,
    USERS,
    WALLOPS,
    USERHOST,
    ISON,
}

#[allow(non_snake_case)]
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &Command::NICK => "NICK",
                &Command::PASS => "PASS",
                &Command::USER => "USER",
                &Command::SERVER => "SERVER",
                &Command::OPER => "OPER",
                &Command::QUIT => "QUIT",
                &Command::SQUIT => "SQUIT",
                &Command::JOIN => "JOIN",
                &Command::PART => "PART",
                &Command::MODE => "MODE",
                &Command::TOPIC => "TOPIC",
                &Command::NAMES => "NAMES",
                &Command::LIST => "LIST",
                &Command::INVITE => "INVITE",
                &Command::KICK => "KICK",
                &Command::VERSION => "VERSION",
                &Command::STATS => "STATS",
                &Command::LINKS => "LINKS",
                &Command::TIME => "TIME",
                &Command::CONNECT => "CONNECT",
                &Command::TRACE => "TRACE",
                &Command::ADMIN => "ADMIN",
                &Command::INFO => "INFO",
                &Command::PRIVMSG => "PRIVMSG",
                &Command::NOTICE => "NOTICE",
                &Command::WHO => "WHO",
                &Command::WHOIS => "WHOIS",
                &Command::WHOWAS => "WHOWAS",
                &Command::KILL => "KILL",
                &Command::PING => "PING",
                &Command::PONG => "PONG",
                &Command::ERROR => "ERROR",
                &Command::AWAY => "AWAY",
                &Command::REHASH => "REHASH",
                &Command::RESTART => "RESTART",
                &Command::SUMMON => "SUMMON",
                &Command::USERS => "USERS",
                &Command::WALLOPS => "WALLOPS",
                &Command::USERHOST => "USERHOST",
                &Command::ISON => "ISON",
            }
        )
    }
}

impl str::FromStr for Command {
    type Err = errors::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_ref() {
            "NICK" => Ok(Command::NICK),
            "PASS" => Ok(Command::PASS),
            "USER" => Ok(Command::USER),
            "SERVER" => Ok(Command::SERVER),
            "OPER" => Ok(Command::OPER),
            "QUIT" => Ok(Command::QUIT),
            "SQUIT" => Ok(Command::SQUIT),
            "JOIN" => Ok(Command::JOIN),
            "PART" => Ok(Command::PART),
            "MODE" => Ok(Command::MODE),
            "TOPIC" => Ok(Command::TOPIC),
            "NAMES" => Ok(Command::NAMES),
            "LIST" => Ok(Command::LIST),
            "INVITE" => Ok(Command::INVITE),
            "KICK" => Ok(Command::KICK),
            "VERSION" => Ok(Command::VERSION),
            "STATS" => Ok(Command::STATS),
            "LINKS" => Ok(Command::LINKS),
            "TIME" => Ok(Command::TIME),
            "CONNECT" => Ok(Command::CONNECT),
            "TRACE" => Ok(Command::TRACE),
            "ADMIN" => Ok(Command::ADMIN),
            "INFO" => Ok(Command::INFO),
            "PRIVMSG" => Ok(Command::PRIVMSG),
            "NOTICE" => Ok(Command::NOTICE),
            "WHO" => Ok(Command::WHO),
            "WHOIS" => Ok(Command::WHOIS),
            "WHOWAS" => Ok(Command::WHOWAS),
            "KILL" => Ok(Command::KILL),
            "PING" => Ok(Command::PING),
            "PONG" => Ok(Command::PONG),
            "ERROR" => Ok(Command::ERROR),
            "AWAY" => Ok(Command::AWAY),
            "REHASH" => Ok(Command::REHASH),
            "RESTART" => Ok(Command::RESTART),
            "SUMMON" => Ok(Command::SUMMON),
            "USERS" => Ok(Command::USERS),
            "WALLOPS" => Ok(Command::WALLOPS),
            "USERHOST" => Ok(Command::USERHOST),
            "ISON" => Ok(Command::ISON),
            _ => Err(errors::ParseError::new("cannot parse command string")),
        }
    }
}

// RFC 1459 6
#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Reply {
    // 6.1 Error replies.
    ERR_NOSUCHNICK = 401,
    ERR_NOSUCHSERVER = 402,
    ERR_NOSUCHCHANNEL = 403,
    ERR_CANNOTSENDTOCHAN = 404,
    ERR_TOOMANYCHANNELS = 405,
    ERR_WASNOSUCHNICK = 406,
    ERR_TOOMANYTARGETS = 407,
    ERR_NOORIGIN = 409,
    ERR_NORECIPIENT = 411,
    ERR_NOTEXTTOSEND = 412,
    ERR_NOTOPLEVEL = 413,
    ERR_WILDTOPLEVEL = 414,
    ERR_UNKNOWNCOMMAND = 421,
    ERR_NOMOTD = 422,
    ERR_NOADMININFO = 423,
    ERR_FILEERROR = 424,
    ERR_NONICKNAMEGIVEN = 431,
    ERR_ERRONEUSNICKNAME = 432,
    ERR_NICKNAMEINUSE = 433,
    ERR_NICKCOLLISION = 436,
    ERR_USERNOTINCHANNEL = 441,
    ERR_NOTONCHANNEL = 442,
    ERR_USERONCHANNEL = 443,
    ERR_NOLOGIN = 444,
    ERR_SUMMONDISABLED = 445,
    ERR_USERSDISABLED = 446,
    ERR_NOTREGISTERED = 451,
    ERR_NEEDMOREPARAMS = 461,
    ERR_ALREADYREGISTRED = 462,
    ERR_NOPERMFORHOST = 463,
    ERR_PASSWDMISMATCH = 464,
    ERR_YOUREBANNEDCREEP = 465,
    ERR_KEYSET = 467,
    ERR_CHANNELISFULL = 471,
    ERR_UNKNOWNMODE = 472,
    ERR_INVITEONLYCHAN = 473,
    ERR_BANNEDFROMCHAN = 474,
    ERR_BADCHANNELKEY = 475,
    ERR_NOPRIVILEGES = 481,
    ERR_CHANOPRIVSNEEDED = 482,
    ERR_CANTKILLSERVER = 483,
    ERR_NOOPERHOST = 491,
    ERR_UMODEUNKNOWNFLAG = 501,
    ERR_USERSDONTMATCH = 502,

    // 6.2 Command responses.
    RPL_NONE = 300,
    RPL_USERHOST = 302,
    RPL_ISON = 303,
    RPL_AWAY = 301,
    RPL_UNAWAY = 305,
    RPL_NOWAWAY = 306,
    RPL_WHOISUSER = 311,
    RPL_WHOISSERVER = 312,
    RPL_WHOISOPERATOR = 313,
    RPL_WHOISIDLE = 317,
    RPL_ENDOFWHOIS = 318,
    RPL_WHOISCHANNELS = 319,
    RPL_WHOWASUSER = 314,
    RPL_ENDOFWHOWAS = 369,
    RPL_LISTSTART = 321,
    RPL_LIST = 322,
    RPL_LISTEND = 323,
    RPL_CHANNELMODEIS = 324,
    RPL_NOTOPIC = 331,
    RPL_TOPIC = 332,
    RPL_INVITING = 341,
    RPL_SUMMONING = 342,
    RPL_VERSION = 351,
    RPL_WHOREPLY = 352,
    RPL_ENDOFWHO = 315,
    RPL_NAMREPLY = 353,
    RPL_ENDOFNAMES = 366,
    RPL_LINKS = 364,
    RPL_ENDOFLINKS = 365,
    RPL_BANLIST = 367,
    RPL_ENDOFBANLIST = 368,
    RPL_INFO = 371,
    RPL_ENDOFINFO = 374,
    RPL_MOTDSTART = 375,
    RPL_MOTD = 372,
    RPL_ENDOFMOTD = 376,
    RPL_YOUREOPER = 381,
    RPL_REHASHING = 382,
    RPL_TIME = 391,
    RPL_USERSSTART = 392,
    RPL_USERS = 393,
    RPL_ENDOFUSERS = 394,
    RPL_NOUSERS = 395,
    RPL_TRACELINK = 200,
    RPL_TRACECONNECTING = 201,
    RPL_TRACEHANDSHAKE = 202,
    RPL_TRACEUNKNOWN = 203,
    RPL_TRACEOPERATOR = 204,
    RPL_TRACEUSER = 205,
    RPL_TRACESERVER = 206,
    RPL_TRACENEWTYPE = 208,
    RPL_TRACELOG = 261,
    RPL_STATSLINKINFO = 211,
    RPL_STATSCOMMANDS = 212,
    RPL_STATSCLINE = 213,
    RPL_STATSNLINE = 214,
    RPL_STATSILINE = 215,
    RPL_STATSKLINE = 216,
    RPL_STATSYLINE = 218,
    RPL_ENDOFSTATS = 219,
    RPL_STATSLLINE = 241,
    RPL_STATSUPTIME = 242,
    RPL_STATSOLINE = 243,
    RPL_STATSHLINE = 244,
    RPL_UMODEIS = 221,
    RPL_LUSERCLIENT = 251,
    RPL_LUSEROP = 252,
    RPL_LUSERUNKNOWN = 253,
    RPL_LUSERCHANNELS = 254,
    RPL_LUSERME = 255,
    RPL_ADMINME = 256,
    RPL_ADMINLOC1 = 257,
    RPL_ADMINLOC2 = 258,
    RPL_ADMINEMAIL = 259,

    // 6.3 Reserved.
    RPL_TRACECLASS = 209,
    RPL_STATSQLINE = 217,
    RPL_SERVICEINFO = 231,
    RPL_ENDOFSERVICES = 232,
    RPL_SERVICE = 233,
    RPL_SERVLIST = 234,
    RPL_SERVLISTEND = 235,
    RPL_WHOISCHANOP = 316,
    RPL_KILLDONE = 361,
    RPL_CLOSING = 362,
    RPL_CLOSEEND = 363,
    RPL_INFOSTART = 373,
    RPL_MYPORTIS = 384,
    ERR_YOUWILLBEBANNED = 466,
    ERR_BADCHANMASK = 476,
    ERR_NOSERVICEHOST = 492,
}

impl fmt::Display for Reply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let o = match self {
            &Reply::ERR_NOSUCHNICK => ("ERR_NOSUCHNICK", 401),
            &Reply::ERR_NOSUCHSERVER => ("ERR_NOSUCHSERVER", 402),
            &Reply::ERR_NOSUCHCHANNEL => ("ERR_NOSUCHCHANNEL", 403),
            &Reply::ERR_CANNOTSENDTOCHAN => ("ERR_CANNOTSENDTOCHAN", 404),
            &Reply::ERR_TOOMANYCHANNELS => ("ERR_TOOMANYCHANNELS", 405),
            &Reply::ERR_WASNOSUCHNICK => ("ERR_WASNOSUCHNICK", 406),
            &Reply::ERR_TOOMANYTARGETS => ("ERR_TOOMANYTARGETS", 407),
            &Reply::ERR_NOORIGIN => ("ERR_NOORIGIN", 409),
            &Reply::ERR_NORECIPIENT => ("ERR_NORECIPIENT", 411),
            &Reply::ERR_NOTEXTTOSEND => ("ERR_NOTEXTTOSEND", 412),
            &Reply::ERR_NOTOPLEVEL => ("ERR_NOTOPLEVEL", 413),
            &Reply::ERR_WILDTOPLEVEL => ("ERR_WILDTOPLEVEL", 414),
            &Reply::ERR_UNKNOWNCOMMAND => ("ERR_UNKNOWNCOMMAND", 421),
            &Reply::ERR_NOMOTD => ("ERR_NOMOTD", 422),
            &Reply::ERR_NOADMININFO => ("ERR_NOADMININFO", 423),
            &Reply::ERR_FILEERROR => ("ERR_FILEERROR", 424),
            &Reply::ERR_NONICKNAMEGIVEN => ("ERR_NONICKNAMEGIVEN", 431),
            &Reply::ERR_ERRONEUSNICKNAME => ("ERR_ERRONEUSNICKNAME", 432),
            &Reply::ERR_NICKNAMEINUSE => ("ERR_NICKNAMEINUSE", 433),
            &Reply::ERR_NICKCOLLISION => ("ERR_NICKCOLLISION", 436),
            &Reply::ERR_USERNOTINCHANNEL => ("ERR_USERNOTINCHANNEL", 441),
            &Reply::ERR_NOTONCHANNEL => ("ERR_NOTONCHANNEL", 442),
            &Reply::ERR_USERONCHANNEL => ("ERR_USERONCHANNEL", 443),
            &Reply::ERR_NOLOGIN => ("ERR_NOLOGIN", 444),
            &Reply::ERR_SUMMONDISABLED => ("ERR_SUMMONDISABLED", 445),
            &Reply::ERR_USERSDISABLED => ("ERR_USERSDISABLED", 446),
            &Reply::ERR_NOTREGISTERED => ("ERR_NOTREGISTERED", 451),
            &Reply::ERR_NEEDMOREPARAMS => ("ERR_NEEDMOREPARAMS", 461),
            &Reply::ERR_ALREADYREGISTRED => ("ERR_ALREADYREGISTRED", 462),
            &Reply::ERR_NOPERMFORHOST => ("ERR_NOPERMFORHOST", 463),
            &Reply::ERR_PASSWDMISMATCH => ("ERR_PASSWDMISMATCH", 464),
            &Reply::ERR_YOUREBANNEDCREEP => ("ERR_YOUREBANNEDCREEP", 465),
            &Reply::ERR_KEYSET => ("ERR_KEYSET", 467),
            &Reply::ERR_CHANNELISFULL => ("ERR_CHANNELISFULL", 471),
            &Reply::ERR_UNKNOWNMODE => ("ERR_UNKNOWNMODE", 472),
            &Reply::ERR_INVITEONLYCHAN => ("ERR_INVITEONLYCHAN", 473),
            &Reply::ERR_BANNEDFROMCHAN => ("ERR_BANNEDFROMCHAN", 474),
            &Reply::ERR_BADCHANNELKEY => ("ERR_BADCHANNELKEY", 475),
            &Reply::ERR_NOPRIVILEGES => ("ERR_NOPRIVILEGES", 481),
            &Reply::ERR_CHANOPRIVSNEEDED => ("ERR_CHANOPRIVSNEEDED", 482),
            &Reply::ERR_CANTKILLSERVER => ("ERR_CANTKILLSERVER", 483),
            &Reply::ERR_NOOPERHOST => ("ERR_NOOPERHOST", 491),
            &Reply::ERR_UMODEUNKNOWNFLAG => ("ERR_UMODEUNKNOWNFLAG", 501),
            &Reply::ERR_USERSDONTMATCH => ("ERR_USERSDONTMATCH", 502),
            &Reply::RPL_NONE => ("RPL_NONE", 300),
            &Reply::RPL_USERHOST => ("RPL_USERHOST", 302),
            &Reply::RPL_ISON => ("RPL_ISON", 303),
            &Reply::RPL_AWAY => ("RPL_AWAY", 301),
            &Reply::RPL_UNAWAY => ("RPL_UNAWAY", 305),
            &Reply::RPL_NOWAWAY => ("RPL_NOWAWAY", 306),
            &Reply::RPL_WHOISUSER => ("RPL_WHOISUSER", 311),
            &Reply::RPL_WHOISSERVER => ("RPL_WHOISSERVER", 312),
            &Reply::RPL_WHOISOPERATOR => ("RPL_WHOISOPERATOR", 313),
            &Reply::RPL_WHOISIDLE => ("RPL_WHOISIDLE", 317),
            &Reply::RPL_ENDOFWHOIS => ("RPL_ENDOFWHOIS", 318),
            &Reply::RPL_WHOISCHANNELS => ("RPL_WHOISCHANNELS", 319),
            &Reply::RPL_WHOWASUSER => ("RPL_WHOWASUSER", 314),
            &Reply::RPL_ENDOFWHOWAS => ("RPL_ENDOFWHOWAS", 369),
            &Reply::RPL_LISTSTART => ("RPL_LISTSTART", 321),
            &Reply::RPL_LIST => ("RPL_LIST", 322),
            &Reply::RPL_LISTEND => ("RPL_LISTEND", 323),
            &Reply::RPL_CHANNELMODEIS => ("RPL_CHANNELMODEIS", 324),
            &Reply::RPL_NOTOPIC => ("RPL_NOTOPIC", 331),
            &Reply::RPL_TOPIC => ("RPL_TOPIC", 332),
            &Reply::RPL_INVITING => ("RPL_INVITING", 341),
            &Reply::RPL_SUMMONING => ("RPL_SUMMONING", 342),
            &Reply::RPL_VERSION => ("RPL_VERSION", 351),
            &Reply::RPL_WHOREPLY => ("RPL_WHOREPLY", 352),
            &Reply::RPL_ENDOFWHO => ("RPL_ENDOFWHO", 315),
            &Reply::RPL_NAMREPLY => ("RPL_NAMREPLY", 353),
            &Reply::RPL_ENDOFNAMES => ("RPL_ENDOFNAMES", 366),
            &Reply::RPL_LINKS => ("RPL_LINKS", 364),
            &Reply::RPL_ENDOFLINKS => ("RPL_ENDOFLINKS", 365),
            &Reply::RPL_BANLIST => ("RPL_BANLIST", 367),
            &Reply::RPL_ENDOFBANLIST => ("RPL_ENDOFBANLIST", 368),
            &Reply::RPL_INFO => ("RPL_INFO", 371),
            &Reply::RPL_ENDOFINFO => ("RPL_ENDOFINFO", 374),
            &Reply::RPL_MOTDSTART => ("RPL_MOTDSTART", 375),
            &Reply::RPL_MOTD => ("RPL_MOTD", 372),
            &Reply::RPL_ENDOFMOTD => ("RPL_ENDOFMOTD", 376),
            &Reply::RPL_YOUREOPER => ("RPL_YOUREOPER", 381),
            &Reply::RPL_REHASHING => ("RPL_REHASHING", 382),
            &Reply::RPL_TIME => ("RPL_TIME", 391),
            &Reply::RPL_USERSSTART => ("RPL_USERSSTART", 392),
            &Reply::RPL_USERS => ("RPL_USERS", 393),
            &Reply::RPL_ENDOFUSERS => ("RPL_ENDOFUSERS", 394),
            &Reply::RPL_NOUSERS => ("RPL_NOUSERS", 395),
            &Reply::RPL_TRACELINK => ("RPL_TRACELINK", 200),
            &Reply::RPL_TRACECONNECTING => ("RPL_TRACECONNECTING", 201),
            &Reply::RPL_TRACEHANDSHAKE => ("RPL_TRACEHANDSHAKE", 202),
            &Reply::RPL_TRACEUNKNOWN => ("RPL_TRACEUNKNOWN", 203),
            &Reply::RPL_TRACEOPERATOR => ("RPL_TRACEOPERATOR", 204),
            &Reply::RPL_TRACEUSER => ("RPL_TRACEUSER", 205),
            &Reply::RPL_TRACESERVER => ("RPL_TRACESERVER", 206),
            &Reply::RPL_TRACENEWTYPE => ("RPL_TRACENEWTYPE", 208),
            &Reply::RPL_TRACELOG => ("RPL_TRACELOG", 261),
            &Reply::RPL_STATSLINKINFO => ("RPL_STATSLINKINFO", 211),
            &Reply::RPL_STATSCOMMANDS => ("RPL_STATSCOMMANDS", 212),
            &Reply::RPL_STATSCLINE => ("RPL_STATSCLINE", 213),
            &Reply::RPL_STATSNLINE => ("RPL_STATSNLINE", 214),
            &Reply::RPL_STATSILINE => ("RPL_STATSILINE", 215),
            &Reply::RPL_STATSKLINE => ("RPL_STATSKLINE", 216),
            &Reply::RPL_STATSYLINE => ("RPL_STATSYLINE", 218),
            &Reply::RPL_ENDOFSTATS => ("RPL_ENDOFSTATS", 219),
            &Reply::RPL_STATSLLINE => ("RPL_STATSLLINE", 241),
            &Reply::RPL_STATSUPTIME => ("RPL_STATSUPTIME", 242),
            &Reply::RPL_STATSOLINE => ("RPL_STATSOLINE", 243),
            &Reply::RPL_STATSHLINE => ("RPL_STATSHLINE", 244),
            &Reply::RPL_UMODEIS => ("RPL_UMODEIS", 221),
            &Reply::RPL_LUSERCLIENT => ("RPL_LUSERCLIENT", 251),
            &Reply::RPL_LUSEROP => ("RPL_LUSEROP", 252),
            &Reply::RPL_LUSERUNKNOWN => ("RPL_LUSERUNKNOWN", 253),
            &Reply::RPL_LUSERCHANNELS => ("RPL_LUSERCHANNELS", 254),
            &Reply::RPL_LUSERME => ("RPL_LUSERME", 255),
            &Reply::RPL_ADMINME => ("RPL_ADMINME", 256),
            &Reply::RPL_ADMINLOC1 => ("RPL_ADMINLOC1", 257),
            &Reply::RPL_ADMINLOC2 => ("RPL_ADMINLOC2", 258),
            &Reply::RPL_ADMINEMAIL => ("RPL_ADMINEMAIL", 259),
            &Reply::RPL_TRACECLASS => ("RPL_TRACECLASS", 209),
            &Reply::RPL_STATSQLINE => ("RPL_STATSQLINE", 217),
            &Reply::RPL_SERVICEINFO => ("RPL_SERVICEINFO", 231),
            &Reply::RPL_ENDOFSERVICES => ("RPL_ENDOFSERVICES", 232),
            &Reply::RPL_SERVICE => ("RPL_SERVICE", 233),
            &Reply::RPL_SERVLIST => ("RPL_SERVLIST", 234),
            &Reply::RPL_SERVLISTEND => ("RPL_SERVLISTEND", 235),
            &Reply::RPL_WHOISCHANOP => ("RPL_WHOISCHANOP", 316),
            &Reply::RPL_KILLDONE => ("RPL_KILLDONE", 361),
            &Reply::RPL_CLOSING => ("RPL_CLOSING", 362),
            &Reply::RPL_CLOSEEND => ("RPL_CLOSEEND", 363),
            &Reply::RPL_INFOSTART => ("RPL_INFOSTART", 373),
            &Reply::RPL_MYPORTIS => ("RPL_MYPORTIS", 384),
            &Reply::ERR_YOUWILLBEBANNED => ("ERR_YOUWILLBEBANNED", 466),
            &Reply::ERR_BADCHANMASK => ("ERR_BADCHANMASK", 476),
            &Reply::ERR_NOSERVICEHOST => ("ERR_NOSERVICEHOST", 492),
        };
        write!(f, "{} {}", o.1, o.0)
    }
}

impl str::FromStr for Reply {
    type Err = errors::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_ref() {
            "ERR_NOSUCHNICK" => Ok(Reply::ERR_NOSUCHNICK),
            "401" => Ok(Reply::ERR_NOSUCHNICK),
            "ERR_NOSUCHSERVER" => Ok(Reply::ERR_NOSUCHSERVER),
            "402" => Ok(Reply::ERR_NOSUCHSERVER),
            "ERR_NOSUCHCHANNEL" => Ok(Reply::ERR_NOSUCHCHANNEL),
            "403" => Ok(Reply::ERR_NOSUCHCHANNEL),
            "ERR_CANNOTSENDTOCHAN" => Ok(Reply::ERR_CANNOTSENDTOCHAN),
            "404" => Ok(Reply::ERR_CANNOTSENDTOCHAN),
            "ERR_TOOMANYCHANNELS" => Ok(Reply::ERR_TOOMANYCHANNELS),
            "405" => Ok(Reply::ERR_TOOMANYCHANNELS),
            "ERR_WASNOSUCHNICK" => Ok(Reply::ERR_WASNOSUCHNICK),
            "406" => Ok(Reply::ERR_WASNOSUCHNICK),
            "ERR_TOOMANYTARGETS" => Ok(Reply::ERR_TOOMANYTARGETS),
            "407" => Ok(Reply::ERR_TOOMANYTARGETS),
            "ERR_NOORIGIN" => Ok(Reply::ERR_NOORIGIN),
            "409" => Ok(Reply::ERR_NOORIGIN),
            "ERR_NORECIPIENT" => Ok(Reply::ERR_NORECIPIENT),
            "411" => Ok(Reply::ERR_NORECIPIENT),
            "ERR_NOTEXTTOSEND" => Ok(Reply::ERR_NOTEXTTOSEND),
            "412" => Ok(Reply::ERR_NOTEXTTOSEND),
            "ERR_NOTOPLEVEL" => Ok(Reply::ERR_NOTOPLEVEL),
            "413" => Ok(Reply::ERR_NOTOPLEVEL),
            "ERR_WILDTOPLEVEL" => Ok(Reply::ERR_WILDTOPLEVEL),
            "414" => Ok(Reply::ERR_WILDTOPLEVEL),
            "ERR_UNKNOWNCOMMAND" => Ok(Reply::ERR_UNKNOWNCOMMAND),
            "421" => Ok(Reply::ERR_UNKNOWNCOMMAND),
            "ERR_NOMOTD" => Ok(Reply::ERR_NOMOTD),
            "422" => Ok(Reply::ERR_NOMOTD),
            "ERR_NOADMININFO" => Ok(Reply::ERR_NOADMININFO),
            "423" => Ok(Reply::ERR_NOADMININFO),
            "ERR_FILEERROR" => Ok(Reply::ERR_FILEERROR),
            "424" => Ok(Reply::ERR_FILEERROR),
            "ERR_NONICKNAMEGIVEN" => Ok(Reply::ERR_NONICKNAMEGIVEN),
            "431" => Ok(Reply::ERR_NONICKNAMEGIVEN),
            "ERR_ERRONEUSNICKNAME" => Ok(Reply::ERR_ERRONEUSNICKNAME),
            "432" => Ok(Reply::ERR_ERRONEUSNICKNAME),
            "ERR_NICKNAMEINUSE" => Ok(Reply::ERR_NICKNAMEINUSE),
            "433" => Ok(Reply::ERR_NICKNAMEINUSE),
            "ERR_NICKCOLLISION" => Ok(Reply::ERR_NICKCOLLISION),
            "436" => Ok(Reply::ERR_NICKCOLLISION),
            "ERR_USERNOTINCHANNEL" => Ok(Reply::ERR_USERNOTINCHANNEL),
            "441" => Ok(Reply::ERR_USERNOTINCHANNEL),
            "ERR_NOTONCHANNEL" => Ok(Reply::ERR_NOTONCHANNEL),
            "442" => Ok(Reply::ERR_NOTONCHANNEL),
            "ERR_USERONCHANNEL" => Ok(Reply::ERR_USERONCHANNEL),
            "443" => Ok(Reply::ERR_USERONCHANNEL),
            "ERR_NOLOGIN" => Ok(Reply::ERR_NOLOGIN),
            "444" => Ok(Reply::ERR_NOLOGIN),
            "ERR_SUMMONDISABLED" => Ok(Reply::ERR_SUMMONDISABLED),
            "445" => Ok(Reply::ERR_SUMMONDISABLED),
            "ERR_USERSDISABLED" => Ok(Reply::ERR_USERSDISABLED),
            "446" => Ok(Reply::ERR_USERSDISABLED),
            "ERR_NOTREGISTERED" => Ok(Reply::ERR_NOTREGISTERED),
            "451" => Ok(Reply::ERR_NOTREGISTERED),
            "ERR_NEEDMOREPARAMS" => Ok(Reply::ERR_NEEDMOREPARAMS),
            "461" => Ok(Reply::ERR_NEEDMOREPARAMS),
            "ERR_ALREADYREGISTRED" => Ok(Reply::ERR_ALREADYREGISTRED),
            "462" => Ok(Reply::ERR_ALREADYREGISTRED),
            "ERR_NOPERMFORHOST" => Ok(Reply::ERR_NOPERMFORHOST),
            "463" => Ok(Reply::ERR_NOPERMFORHOST),
            "ERR_PASSWDMISMATCH" => Ok(Reply::ERR_PASSWDMISMATCH),
            "464" => Ok(Reply::ERR_PASSWDMISMATCH),
            "ERR_YOUREBANNEDCREEP" => Ok(Reply::ERR_YOUREBANNEDCREEP),
            "465" => Ok(Reply::ERR_YOUREBANNEDCREEP),
            "ERR_KEYSET" => Ok(Reply::ERR_KEYSET),
            "467" => Ok(Reply::ERR_KEYSET),
            "ERR_CHANNELISFULL" => Ok(Reply::ERR_CHANNELISFULL),
            "471" => Ok(Reply::ERR_CHANNELISFULL),
            "ERR_UNKNOWNMODE" => Ok(Reply::ERR_UNKNOWNMODE),
            "472" => Ok(Reply::ERR_UNKNOWNMODE),
            "ERR_INVITEONLYCHAN" => Ok(Reply::ERR_INVITEONLYCHAN),
            "473" => Ok(Reply::ERR_INVITEONLYCHAN),
            "ERR_BANNEDFROMCHAN" => Ok(Reply::ERR_BANNEDFROMCHAN),
            "474" => Ok(Reply::ERR_BANNEDFROMCHAN),
            "ERR_BADCHANNELKEY" => Ok(Reply::ERR_BADCHANNELKEY),
            "475" => Ok(Reply::ERR_BADCHANNELKEY),
            "ERR_NOPRIVILEGES" => Ok(Reply::ERR_NOPRIVILEGES),
            "481" => Ok(Reply::ERR_NOPRIVILEGES),
            "ERR_CHANOPRIVSNEEDED" => Ok(Reply::ERR_CHANOPRIVSNEEDED),
            "482" => Ok(Reply::ERR_CHANOPRIVSNEEDED),
            "ERR_CANTKILLSERVER" => Ok(Reply::ERR_CANTKILLSERVER),
            "483" => Ok(Reply::ERR_CANTKILLSERVER),
            "ERR_NOOPERHOST" => Ok(Reply::ERR_NOOPERHOST),
            "491" => Ok(Reply::ERR_NOOPERHOST),
            "ERR_UMODEUNKNOWNFLAG" => Ok(Reply::ERR_UMODEUNKNOWNFLAG),
            "501" => Ok(Reply::ERR_UMODEUNKNOWNFLAG),
            "ERR_USERSDONTMATCH" => Ok(Reply::ERR_USERSDONTMATCH),
            "502" => Ok(Reply::ERR_USERSDONTMATCH),
            "RPL_NONE" => Ok(Reply::RPL_NONE),
            "300" => Ok(Reply::RPL_NONE),
            "RPL_USERHOST" => Ok(Reply::RPL_USERHOST),
            "302" => Ok(Reply::RPL_USERHOST),
            "RPL_ISON" => Ok(Reply::RPL_ISON),
            "303" => Ok(Reply::RPL_ISON),
            "RPL_AWAY" => Ok(Reply::RPL_AWAY),
            "301" => Ok(Reply::RPL_AWAY),
            "RPL_UNAWAY" => Ok(Reply::RPL_UNAWAY),
            "305" => Ok(Reply::RPL_UNAWAY),
            "RPL_NOWAWAY" => Ok(Reply::RPL_NOWAWAY),
            "306" => Ok(Reply::RPL_NOWAWAY),
            "RPL_WHOISUSER" => Ok(Reply::RPL_WHOISUSER),
            "311" => Ok(Reply::RPL_WHOISUSER),
            "RPL_WHOISSERVER" => Ok(Reply::RPL_WHOISSERVER),
            "312" => Ok(Reply::RPL_WHOISSERVER),
            "RPL_WHOISOPERATOR" => Ok(Reply::RPL_WHOISOPERATOR),
            "313" => Ok(Reply::RPL_WHOISOPERATOR),
            "RPL_WHOISIDLE" => Ok(Reply::RPL_WHOISIDLE),
            "317" => Ok(Reply::RPL_WHOISIDLE),
            "RPL_ENDOFWHOIS" => Ok(Reply::RPL_ENDOFWHOIS),
            "318" => Ok(Reply::RPL_ENDOFWHOIS),
            "RPL_WHOISCHANNELS" => Ok(Reply::RPL_WHOISCHANNELS),
            "319" => Ok(Reply::RPL_WHOISCHANNELS),
            "RPL_WHOWASUSER" => Ok(Reply::RPL_WHOWASUSER),
            "314" => Ok(Reply::RPL_WHOWASUSER),
            "RPL_ENDOFWHOWAS" => Ok(Reply::RPL_ENDOFWHOWAS),
            "369" => Ok(Reply::RPL_ENDOFWHOWAS),
            "RPL_LISTSTART" => Ok(Reply::RPL_LISTSTART),
            "321" => Ok(Reply::RPL_LISTSTART),
            "RPL_LIST" => Ok(Reply::RPL_LIST),
            "322" => Ok(Reply::RPL_LIST),
            "RPL_LISTEND" => Ok(Reply::RPL_LISTEND),
            "323" => Ok(Reply::RPL_LISTEND),
            "RPL_CHANNELMODEIS" => Ok(Reply::RPL_CHANNELMODEIS),
            "324" => Ok(Reply::RPL_CHANNELMODEIS),
            "RPL_NOTOPIC" => Ok(Reply::RPL_NOTOPIC),
            "331" => Ok(Reply::RPL_NOTOPIC),
            "RPL_TOPIC" => Ok(Reply::RPL_TOPIC),
            "332" => Ok(Reply::RPL_TOPIC),
            "RPL_INVITING" => Ok(Reply::RPL_INVITING),
            "341" => Ok(Reply::RPL_INVITING),
            "RPL_SUMMONING" => Ok(Reply::RPL_SUMMONING),
            "342" => Ok(Reply::RPL_SUMMONING),
            "RPL_VERSION" => Ok(Reply::RPL_VERSION),
            "351" => Ok(Reply::RPL_VERSION),
            "RPL_WHOREPLY" => Ok(Reply::RPL_WHOREPLY),
            "352" => Ok(Reply::RPL_WHOREPLY),
            "RPL_ENDOFWHO" => Ok(Reply::RPL_ENDOFWHO),
            "315" => Ok(Reply::RPL_ENDOFWHO),
            "RPL_NAMREPLY" => Ok(Reply::RPL_NAMREPLY),
            "353" => Ok(Reply::RPL_NAMREPLY),
            "RPL_ENDOFNAMES" => Ok(Reply::RPL_ENDOFNAMES),
            "366" => Ok(Reply::RPL_ENDOFNAMES),
            "RPL_LINKS" => Ok(Reply::RPL_LINKS),
            "364" => Ok(Reply::RPL_LINKS),
            "RPL_ENDOFLINKS" => Ok(Reply::RPL_ENDOFLINKS),
            "365" => Ok(Reply::RPL_ENDOFLINKS),
            "RPL_BANLIST" => Ok(Reply::RPL_BANLIST),
            "367" => Ok(Reply::RPL_BANLIST),
            "RPL_ENDOFBANLIST" => Ok(Reply::RPL_ENDOFBANLIST),
            "368" => Ok(Reply::RPL_ENDOFBANLIST),
            "RPL_INFO" => Ok(Reply::RPL_INFO),
            "371" => Ok(Reply::RPL_INFO),
            "RPL_ENDOFINFO" => Ok(Reply::RPL_ENDOFINFO),
            "374" => Ok(Reply::RPL_ENDOFINFO),
            "RPL_MOTDSTART" => Ok(Reply::RPL_MOTDSTART),
            "375" => Ok(Reply::RPL_MOTDSTART),
            "RPL_MOTD" => Ok(Reply::RPL_MOTD),
            "372" => Ok(Reply::RPL_MOTD),
            "RPL_ENDOFMOTD" => Ok(Reply::RPL_ENDOFMOTD),
            "376" => Ok(Reply::RPL_ENDOFMOTD),
            "RPL_YOUREOPER" => Ok(Reply::RPL_YOUREOPER),
            "381" => Ok(Reply::RPL_YOUREOPER),
            "RPL_REHASHING" => Ok(Reply::RPL_REHASHING),
            "382" => Ok(Reply::RPL_REHASHING),
            "RPL_TIME" => Ok(Reply::RPL_TIME),
            "391" => Ok(Reply::RPL_TIME),
            "RPL_USERSSTART" => Ok(Reply::RPL_USERSSTART),
            "392" => Ok(Reply::RPL_USERSSTART),
            "RPL_USERS" => Ok(Reply::RPL_USERS),
            "393" => Ok(Reply::RPL_USERS),
            "RPL_ENDOFUSERS" => Ok(Reply::RPL_ENDOFUSERS),
            "394" => Ok(Reply::RPL_ENDOFUSERS),
            "RPL_NOUSERS" => Ok(Reply::RPL_NOUSERS),
            "395" => Ok(Reply::RPL_NOUSERS),
            "RPL_TRACELINK" => Ok(Reply::RPL_TRACELINK),
            "200" => Ok(Reply::RPL_TRACELINK),
            "RPL_TRACECONNECTING" => Ok(Reply::RPL_TRACECONNECTING),
            "201" => Ok(Reply::RPL_TRACECONNECTING),
            "RPL_TRACEHANDSHAKE" => Ok(Reply::RPL_TRACEHANDSHAKE),
            "202" => Ok(Reply::RPL_TRACEHANDSHAKE),
            "RPL_TRACEUNKNOWN" => Ok(Reply::RPL_TRACEUNKNOWN),
            "203" => Ok(Reply::RPL_TRACEUNKNOWN),
            "RPL_TRACEOPERATOR" => Ok(Reply::RPL_TRACEOPERATOR),
            "204" => Ok(Reply::RPL_TRACEOPERATOR),
            "RPL_TRACEUSER" => Ok(Reply::RPL_TRACEUSER),
            "205" => Ok(Reply::RPL_TRACEUSER),
            "RPL_TRACESERVER" => Ok(Reply::RPL_TRACESERVER),
            "206" => Ok(Reply::RPL_TRACESERVER),
            "RPL_TRACENEWTYPE" => Ok(Reply::RPL_TRACENEWTYPE),
            "208" => Ok(Reply::RPL_TRACENEWTYPE),
            "RPL_TRACELOG" => Ok(Reply::RPL_TRACELOG),
            "261" => Ok(Reply::RPL_TRACELOG),
            "RPL_STATSLINKINFO" => Ok(Reply::RPL_STATSLINKINFO),
            "211" => Ok(Reply::RPL_STATSLINKINFO),
            "RPL_STATSCOMMANDS" => Ok(Reply::RPL_STATSCOMMANDS),
            "212" => Ok(Reply::RPL_STATSCOMMANDS),
            "RPL_STATSCLINE" => Ok(Reply::RPL_STATSCLINE),
            "213" => Ok(Reply::RPL_STATSCLINE),
            "RPL_STATSNLINE" => Ok(Reply::RPL_STATSNLINE),
            "214" => Ok(Reply::RPL_STATSNLINE),
            "RPL_STATSILINE" => Ok(Reply::RPL_STATSILINE),
            "215" => Ok(Reply::RPL_STATSILINE),
            "RPL_STATSKLINE" => Ok(Reply::RPL_STATSKLINE),
            "216" => Ok(Reply::RPL_STATSKLINE),
            "RPL_STATSYLINE" => Ok(Reply::RPL_STATSYLINE),
            "218" => Ok(Reply::RPL_STATSYLINE),
            "RPL_ENDOFSTATS" => Ok(Reply::RPL_ENDOFSTATS),
            "219" => Ok(Reply::RPL_ENDOFSTATS),
            "RPL_STATSLLINE" => Ok(Reply::RPL_STATSLLINE),
            "241" => Ok(Reply::RPL_STATSLLINE),
            "RPL_STATSUPTIME" => Ok(Reply::RPL_STATSUPTIME),
            "242" => Ok(Reply::RPL_STATSUPTIME),
            "RPL_STATSOLINE" => Ok(Reply::RPL_STATSOLINE),
            "243" => Ok(Reply::RPL_STATSOLINE),
            "RPL_STATSHLINE" => Ok(Reply::RPL_STATSHLINE),
            "244" => Ok(Reply::RPL_STATSHLINE),
            "RPL_UMODEIS" => Ok(Reply::RPL_UMODEIS),
            "221" => Ok(Reply::RPL_UMODEIS),
            "RPL_LUSERCLIENT" => Ok(Reply::RPL_LUSERCLIENT),
            "251" => Ok(Reply::RPL_LUSERCLIENT),
            "RPL_LUSEROP" => Ok(Reply::RPL_LUSEROP),
            "252" => Ok(Reply::RPL_LUSEROP),
            "RPL_LUSERUNKNOWN" => Ok(Reply::RPL_LUSERUNKNOWN),
            "253" => Ok(Reply::RPL_LUSERUNKNOWN),
            "RPL_LUSERCHANNELS" => Ok(Reply::RPL_LUSERCHANNELS),
            "254" => Ok(Reply::RPL_LUSERCHANNELS),
            "RPL_LUSERME" => Ok(Reply::RPL_LUSERME),
            "255" => Ok(Reply::RPL_LUSERME),
            "RPL_ADMINME" => Ok(Reply::RPL_ADMINME),
            "256" => Ok(Reply::RPL_ADMINME),
            "RPL_ADMINLOC1" => Ok(Reply::RPL_ADMINLOC1),
            "257" => Ok(Reply::RPL_ADMINLOC1),
            "RPL_ADMINLOC2" => Ok(Reply::RPL_ADMINLOC2),
            "258" => Ok(Reply::RPL_ADMINLOC2),
            "RPL_ADMINEMAIL" => Ok(Reply::RPL_ADMINEMAIL),
            "259" => Ok(Reply::RPL_ADMINEMAIL),
            "RPL_TRACECLASS" => Ok(Reply::RPL_TRACECLASS),
            "209" => Ok(Reply::RPL_TRACECLASS),
            "RPL_STATSQLINE" => Ok(Reply::RPL_STATSQLINE),
            "217" => Ok(Reply::RPL_STATSQLINE),
            "RPL_SERVICEINFO" => Ok(Reply::RPL_SERVICEINFO),
            "231" => Ok(Reply::RPL_SERVICEINFO),
            "RPL_ENDOFSERVICES" => Ok(Reply::RPL_ENDOFSERVICES),
            "232" => Ok(Reply::RPL_ENDOFSERVICES),
            "RPL_SERVICE" => Ok(Reply::RPL_SERVICE),
            "233" => Ok(Reply::RPL_SERVICE),
            "RPL_SERVLIST" => Ok(Reply::RPL_SERVLIST),
            "234" => Ok(Reply::RPL_SERVLIST),
            "RPL_SERVLISTEND" => Ok(Reply::RPL_SERVLISTEND),
            "235" => Ok(Reply::RPL_SERVLISTEND),
            "RPL_WHOISCHANOP" => Ok(Reply::RPL_WHOISCHANOP),
            "316" => Ok(Reply::RPL_WHOISCHANOP),
            "RPL_KILLDONE" => Ok(Reply::RPL_KILLDONE),
            "361" => Ok(Reply::RPL_KILLDONE),
            "RPL_CLOSING" => Ok(Reply::RPL_CLOSING),
            "362" => Ok(Reply::RPL_CLOSING),
            "RPL_CLOSEEND" => Ok(Reply::RPL_CLOSEEND),
            "363" => Ok(Reply::RPL_CLOSEEND),
            "RPL_INFOSTART" => Ok(Reply::RPL_INFOSTART),
            "373" => Ok(Reply::RPL_INFOSTART),
            "RPL_MYPORTIS" => Ok(Reply::RPL_MYPORTIS),
            "384" => Ok(Reply::RPL_MYPORTIS),
            "ERR_YOUWILLBEBANNED" => Ok(Reply::ERR_YOUWILLBEBANNED),
            "466" => Ok(Reply::ERR_YOUWILLBEBANNED),
            "ERR_BADCHANMASK" => Ok(Reply::ERR_BADCHANMASK),
            "476" => Ok(Reply::ERR_BADCHANMASK),
            "ERR_NOSERVICEHOST" => Ok(Reply::ERR_NOSERVICEHOST),
            "492" => Ok(Reply::ERR_NOSERVICEHOST),
            _ => Err(errors::ParseError::new("cannot parse reply string")),
        }
    }
}

pub fn parse_command(input: &String) -> Result<Message, errors::ParseError> {
    let syntax = parse_syntax(input)?;
    print!("{}", Reply::RPL_CLOSING as i32);
    Ok(Message {
        prefix: syntax.prefix,
        command: syntax.command,
        params: syntax.params,
    })
}

// RFC 1459 2
fn parse_syntax(input: &String) -> Result<Syntax, errors::ParseError> {
    if input.len() < 2 || input.len() > 512 {
        return Err(errors::ParseError::new("bad command length"));
    }
    if !input.ends_with("\r\n") {
        return Err(errors::ParseError::new("command doesn't end with CR LF"));
    }

    let mut remainder: &str = &input.trim_right();
    debug!("Processing {:?}", remainder);

    let mut prefix: Option<String> = None;
    if remainder.starts_with(':') {
        match remainder.find(' ') {
            Some(idx) => {
                prefix = Some(remainder[0..idx].to_string());
                remainder = &remainder[idx + 1..];
            }
            None => {
                return Err(errors::ParseError::new("only command prefix given"));
            }
        }
    }

    if remainder.len() < 1 {
        return Err(errors::ParseError::new("no command specified"));
    }
    let command: String;
    match remainder.find(' ') {
        Some(idx) => {
            command = remainder[0..idx].to_string();
            remainder = &remainder[idx + 1..];
        }
        None => {
            command = remainder.to_string();
            remainder = "";
        }
    }

    let mut params: Vec<String> = Vec::new();
    while remainder.len() > 0 {
        if remainder.starts_with(':') {
            if remainder.len() == 1 {
                warn!("Empty trailing command parameter. Ignoring.")
            } else {
                params.push(remainder[1..].to_string());
            }
            break;
        }
        match remainder.find(' ') {
            Some(idx) => {
                if idx == 0 {
                    warn!("Empty whitespace in command paramter detected! Ignoring.");
                } else {
                    params.push(remainder[0..idx].to_string());
                }
                remainder = &remainder[idx + 1..];
            }
            None => {
                params.push(remainder.to_string());
                break;
            }
        }
    }

    debug!(
        "Parsed {} to prefix: [{:?}]; command: [{}]; params: [{:?}].",
        input,
        prefix,
        command,
        params
    );

    Ok(Syntax {
        prefix: prefix,
        command: command,
        params: params,
    })
}

#[cfg(test)]
mod test {
    use super::parse_syntax;

    macro_rules! test_syntax_fail {
        ($name:ident, $s:expr) => {
            #[test]
            fn $name() {
                assert!(parse_syntax(&format!("{}\r\n", $s)).is_err());
            }
        }
    }
    macro_rules! test_syntax_pass {
        ($name:ident, $input:expr, Syntax {
            prefix: $prefix:expr,
            command: $command:expr,
            params: [$($params:expr),*],
        }) => {
            #[test]
            fn $name() {
                let s = parse_syntax(&format!("{}\r\n",$input)).unwrap();
                let pf = $prefix.to_string();
                if pf.len() == 0 {
                    assert!(s.prefix.is_none());
                } else {
                    assert_eq!(s.prefix.unwrap(), $prefix.to_string());
                }
                assert_eq!(s.command, $command.to_string());
                let params:Vec<&str> = vec![$($params),*];
                let expect :Vec<String> = params.iter().map(|s| s.to_string()).collect();
                assert_eq!(expect.len(), s.params.len());
                expect.iter().zip(s.params.iter()).for_each(|p| assert_eq!(p.0, p.1));
            }
        }
    }

    test_syntax_fail!(empty, "");
    test_syntax_fail!(just_prefix, ":lazau");

    test_syntax_pass!(
        hello_world,
        "hello world",
        Syntax {
            prefix: "",
            command: "hello",
            params: ["world"],
        }
    );
    test_syntax_pass!(
        empty_param,
        "comm",
        Syntax {
            prefix: "",
            command: "comm",
            params: [],
        }
    );
    test_syntax_pass!(
        empty_param_trailer,
        "hello :",
        Syntax {
            prefix: "",
            command: "hello",
            params: [],
        }
    );
    test_syntax_pass!(
        full,
        ":lazau CONNECT server server2 :server 3 5 6",
        Syntax {
            prefix: ":lazau",
            command: "CONNECT",
            params: ["server", "server2", "server 3 5 6"],
        }
    );
}
