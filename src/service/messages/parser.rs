use std::{self, fmt, str};

use super::{Message, Request, Response, Command, UserMode};

#[derive(Debug)]
pub enum ParseErrorKind {
    NoCommand,
    UnrecognizedCommand,
    NeedMoreParams,
    TooManyParams,
    ParseIntError,
    NotARequest,
    NotAResponse,
    Other,
}

#[derive(Debug)]
pub struct ParseError {
    desc: &'static str,
    pub kind: ParseErrorKind,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, desc: &'static str) -> ParseError {
        ParseError {
            desc: desc,
            kind: kind,
        }
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        self.desc
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error: {}", &self.desc)
    }
}

fn next_token<'a>(s: &'a str) -> (&'a str, &'a str) {
    match s.find(' ') {
        Some(idx) => {
            let (a, b) = s.split_at(idx);
            (a, &b[1..])
        }
        None => (s, ""),
    }
}

impl str::FromStr for Message {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        trace!("Parsing {} to Message.", s);

        let mut remainder: &str = &s;
        let mut prefix = None;
        if s.starts_with(":") {
            let (a, b) = next_token(s);
            if b.len() == 0 {
                return Err(ParseError::new(ParseErrorKind::NoCommand, "no command"));
            }
            remainder = b;
            prefix = Some(a[1..].to_string());
        }

        let command = remainder.parse::<Command>()?;

        Ok(Message {
            prefix: prefix,
            command: command,
        })
    }
}

impl str::FromStr for Command {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let request;
        match s.parse::<Request>() {
            Ok(r) => request = Some(r),
            Err(e) => {
                match e.kind {
                    ParseErrorKind::NotARequest => request = None,
                    _ => return Err(e),
                };
            }
        };

        if let Some(r) = request {
            return Ok(Command::Req(r));
        }

        let response;
        match s.parse::<Response>() {
            Ok(r) => response = Some(r),
            Err(e) => {
                match e.kind {
                    ParseErrorKind::NotAResponse => response = None,
                    _ => return Err(e),
                };
            }
        };

        if let Some(r) = response {
            return Ok(Command::Resp(r));
        }

        return Err(ParseError::new(
            ParseErrorKind::UnrecognizedCommand,
            "not a command",
        ));
    }
}

fn verify_at_least_params<'a>(
    p: &Vec<&'a str>,
    required: usize,
    error: &'static str,
) -> Result<(), ParseError> {
    if p.len() < required {
        return Err(ParseError::new(ParseErrorKind::NeedMoreParams, error));
    }
    Ok(())
}

impl str::FromStr for Request {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut remainder: &str = &s;

        let (command, mut remainder) = next_token(s);

        let mut params: Vec<&str> = Vec::new();
        while remainder.len() > 0 {
            if remainder.starts_with(':') {
                if remainder.len() == 1 {
                    warn!("Empty trailing command parameter. Ignoring.")
                } else {
                    params.push(&remainder[1..]);
                }
                break;
            }

            let (next_param, r) = next_token(s);
            remainder = r;

            if next_param.len() == 0 {
                warn!("Empty whitespace in command paramter detected! Ignoring.");
            } else {
                params.push(next_param);
            }
        }

        match command.to_uppercase().as_ref() {
            "NICK" => {
                verify_at_least_params(&params, 1, "NICK")?;
                Ok(Request::NICK { nickname: params[0].to_string() })
            }
            "PASS" => {
                verify_at_least_params(&params, 1, "PASS")?;
                Ok(Request::PASS { password: params[0].to_string() })
            }
            "USER" => {
                verify_at_least_params(&params, 4, "USER")?;
                Ok(Request::USER {
                    username: params[0].to_string(),
                    mode: params[1].parse::<UserMode>()?,
                    unused: params[2].to_string(),
                    realname: params[3].to_string(),
                })
            }
            "SERVER" => {
                verify_at_least_params(&params, 3, "USER")?;
                Ok(Request::SERVER {
                    servername: params[0].to_string(),
                    hopcount: match params[1].parse::<u64>() {
                        Ok(h) => h,
                        Err(_) => {
                            return Err(ParseError::new(
                                ParseErrorKind::ParseIntError,
                                "hopcount not an int",
                            ))
                        }
                    },
                    token: unimplemented!(),
                    info: params[2].to_string(),
                })
            }
            "OPER" => {
                verify_at_least_params(&params, 2, "OPER")?;
                Ok(Request::OPER {
                    name: params[0].to_string(),
                    password: params[1].to_string(),
                })
            }
            "QUIT" => {
                if params.len() == 0 {
                    return Ok(Request::QUIT { message: None });
                }
                Ok(Request::QUIT { message: Some(params[0].to_string()) })
            }
            /*"SQUIT" => Ok(Request::SQUIT),
            "JOIN" => Ok(Request::JOIN),
            "PART" => Ok(Request::PART),
            "MODE" => Ok(Request::MODE),
            "TOPIC" => Ok(Request::TOPIC),
            "NAMES" => Ok(Request::NAMES),
            "LIST" => Ok(Request::LIST),
            "INVITE" => Ok(Request::INVITE),
            "KICK" => Ok(Request::KICK),
            "VERSION" => Ok(Request::VERSION),
            "STATS" => Ok(Request::STATS),
            "LINKS" => Ok(Request::LINKS),
            "TIME" => Ok(Request::TIME),
            "CONNECT" => Ok(Request::CONNECT),
            "TRACE" => Ok(Request::TRACE),
            "ADMIN" => Ok(Request::ADMIN),
            "INFO" => Ok(Request::INFO),
            "PRIVMSG" => Ok(Request::PRIVMSG),
            "NOTICE" => Ok(Request::NOTICE),
            "WHO" => Ok(Request::WHO),
            "WHOIS" => Ok(Request::WHOIS),
            "WHOWAS" => Ok(Request::WHOWAS),
            "KILL" => Ok(Request::KILL),
            "PING" => Ok(Request::PING),
            "PONG" => Ok(Request::PONG),
            "ERROR" => Ok(Request::ERROR),
            "AWAY" => Ok(Request::AWAY),
            "REHASH" => Ok(Request::REHASH),
            "RESTART" => Ok(Request::RESTART),
            "SUMMON" => Ok(Request::SUMMON),
            "USERS" => Ok(Request::USERS),
            "WALLOPS" => Ok(Request::WALLOPS),
            "USERHOST" => Ok(Request::USERHOST),
            "ISON" => Ok(Request::ISON),*/
            _ => Err(ParseError::new(
                ParseErrorKind::UnrecognizedCommand,
                "unrecognized command",
            )),
            _ => unimplemented!(),
        }
    }
}

impl str::FromStr for Response {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (resp, rem) = next_token(s);
        if rem.len() > 0 {
            unimplemented!()
        }

        match resp.to_uppercase().as_ref() {
            "ERR_NOSUCHNICK" => Ok(Response::ERR_NOSUCHNICK),
            "401" => Ok(Response::ERR_NOSUCHNICK),
            "ERR_NOSUCHSERVER" => Ok(Response::ERR_NOSUCHSERVER),
            "402" => Ok(Response::ERR_NOSUCHSERVER),
            "ERR_NOSUCHCHANNEL" => Ok(Response::ERR_NOSUCHCHANNEL),
            "403" => Ok(Response::ERR_NOSUCHCHANNEL),
            "ERR_CANNOTSENDTOCHAN" => Ok(Response::ERR_CANNOTSENDTOCHAN),
            "404" => Ok(Response::ERR_CANNOTSENDTOCHAN),
            "ERR_TOOMANYCHANNELS" => Ok(Response::ERR_TOOMANYCHANNELS),
            "405" => Ok(Response::ERR_TOOMANYCHANNELS),
            "ERR_WASNOSUCHNICK" => Ok(Response::ERR_WASNOSUCHNICK),
            "406" => Ok(Response::ERR_WASNOSUCHNICK),
            "ERR_TOOMANYTARGETS" => Ok(Response::ERR_TOOMANYTARGETS),
            "407" => Ok(Response::ERR_TOOMANYTARGETS),
            "ERR_NOORIGIN" => Ok(Response::ERR_NOORIGIN),
            "409" => Ok(Response::ERR_NOORIGIN),
            "ERR_NORECIPIENT" => Ok(Response::ERR_NORECIPIENT),
            "411" => Ok(Response::ERR_NORECIPIENT),
            "ERR_NOTEXTTOSEND" => Ok(Response::ERR_NOTEXTTOSEND),
            "412" => Ok(Response::ERR_NOTEXTTOSEND),
            "ERR_NOTOPLEVEL" => Ok(Response::ERR_NOTOPLEVEL),
            "413" => Ok(Response::ERR_NOTOPLEVEL),
            "ERR_WILDTOPLEVEL" => Ok(Response::ERR_WILDTOPLEVEL),
            "414" => Ok(Response::ERR_WILDTOPLEVEL),
            "ERR_UNKNOWNCOMMAND" => Ok(Response::ERR_UNKNOWNCOMMAND),
            "421" => Ok(Response::ERR_UNKNOWNCOMMAND),
            "ERR_NOMOTD" => Ok(Response::ERR_NOMOTD),
            "422" => Ok(Response::ERR_NOMOTD),
            "ERR_NOADMININFO" => Ok(Response::ERR_NOADMININFO),
            "423" => Ok(Response::ERR_NOADMININFO),
            "ERR_FILEERROR" => Ok(Response::ERR_FILEERROR),
            "424" => Ok(Response::ERR_FILEERROR),
            "ERR_NONICKNAMEGIVEN" => Ok(Response::ERR_NONICKNAMEGIVEN),
            "431" => Ok(Response::ERR_NONICKNAMEGIVEN),
            "ERR_ERRONEUSNICKNAME" => Ok(Response::ERR_ERRONEUSNICKNAME),
            "432" => Ok(Response::ERR_ERRONEUSNICKNAME),
            "ERR_NICKNAMEINUSE" => Ok(Response::ERR_NICKNAMEINUSE),
            "433" => Ok(Response::ERR_NICKNAMEINUSE),
            "ERR_NICKCOLLISION" => Ok(Response::ERR_NICKCOLLISION),
            "436" => Ok(Response::ERR_NICKCOLLISION),
            "ERR_USERNOTINCHANNEL" => Ok(Response::ERR_USERNOTINCHANNEL),
            "441" => Ok(Response::ERR_USERNOTINCHANNEL),
            "ERR_NOTONCHANNEL" => Ok(Response::ERR_NOTONCHANNEL),
            "442" => Ok(Response::ERR_NOTONCHANNEL),
            "ERR_USERONCHANNEL" => Ok(Response::ERR_USERONCHANNEL),
            "443" => Ok(Response::ERR_USERONCHANNEL),
            "ERR_NOLOGIN" => Ok(Response::ERR_NOLOGIN),
            "444" => Ok(Response::ERR_NOLOGIN),
            "ERR_SUMMONDISABLED" => Ok(Response::ERR_SUMMONDISABLED),
            "445" => Ok(Response::ERR_SUMMONDISABLED),
            "ERR_USERSDISABLED" => Ok(Response::ERR_USERSDISABLED),
            "446" => Ok(Response::ERR_USERSDISABLED),
            "ERR_NOTREGISTERED" => Ok(Response::ERR_NOTREGISTERED),
            "451" => Ok(Response::ERR_NOTREGISTERED),
            "ERR_NEEDMOREPARAMS" => Ok(Response::ERR_NEEDMOREPARAMS),
            "461" => Ok(Response::ERR_NEEDMOREPARAMS),
            "ERR_ALREADYREGISTRED" => Ok(Response::ERR_ALREADYREGISTRED),
            "462" => Ok(Response::ERR_ALREADYREGISTRED),
            "ERR_NOPERMFORHOST" => Ok(Response::ERR_NOPERMFORHOST),
            "463" => Ok(Response::ERR_NOPERMFORHOST),
            "ERR_PASSWDMISMATCH" => Ok(Response::ERR_PASSWDMISMATCH),
            "464" => Ok(Response::ERR_PASSWDMISMATCH),
            "ERR_YOUREBANNEDCREEP" => Ok(Response::ERR_YOUREBANNEDCREEP),
            "465" => Ok(Response::ERR_YOUREBANNEDCREEP),
            "ERR_KEYSET" => Ok(Response::ERR_KEYSET),
            "467" => Ok(Response::ERR_KEYSET),
            "ERR_CHANNELISFULL" => Ok(Response::ERR_CHANNELISFULL),
            "471" => Ok(Response::ERR_CHANNELISFULL),
            "ERR_UNKNOWNMODE" => Ok(Response::ERR_UNKNOWNMODE),
            "472" => Ok(Response::ERR_UNKNOWNMODE),
            "ERR_INVITEONLYCHAN" => Ok(Response::ERR_INVITEONLYCHAN),
            "473" => Ok(Response::ERR_INVITEONLYCHAN),
            "ERR_BANNEDFROMCHAN" => Ok(Response::ERR_BANNEDFROMCHAN),
            "474" => Ok(Response::ERR_BANNEDFROMCHAN),
            "ERR_BADCHANNELKEY" => Ok(Response::ERR_BADCHANNELKEY),
            "475" => Ok(Response::ERR_BADCHANNELKEY),
            "ERR_NOPRIVILEGES" => Ok(Response::ERR_NOPRIVILEGES),
            "481" => Ok(Response::ERR_NOPRIVILEGES),
            "ERR_CHANOPRIVSNEEDED" => Ok(Response::ERR_CHANOPRIVSNEEDED),
            "482" => Ok(Response::ERR_CHANOPRIVSNEEDED),
            "ERR_CANTKILLSERVER" => Ok(Response::ERR_CANTKILLSERVER),
            "483" => Ok(Response::ERR_CANTKILLSERVER),
            "ERR_NOOPERHOST" => Ok(Response::ERR_NOOPERHOST),
            "491" => Ok(Response::ERR_NOOPERHOST),
            "ERR_UMODEUNKNOWNFLAG" => Ok(Response::ERR_UMODEUNKNOWNFLAG),
            "501" => Ok(Response::ERR_UMODEUNKNOWNFLAG),
            "ERR_USERSDONTMATCH" => Ok(Response::ERR_USERSDONTMATCH),
            "502" => Ok(Response::ERR_USERSDONTMATCH),
            "RPL_NONE" => Ok(Response::RPL_NONE),
            "300" => Ok(Response::RPL_NONE),
            "RPL_USERHOST" => Ok(Response::RPL_USERHOST),
            "302" => Ok(Response::RPL_USERHOST),
            "RPL_ISON" => Ok(Response::RPL_ISON),
            "303" => Ok(Response::RPL_ISON),
            "RPL_AWAY" => Ok(Response::RPL_AWAY),
            "301" => Ok(Response::RPL_AWAY),
            "RPL_UNAWAY" => Ok(Response::RPL_UNAWAY),
            "305" => Ok(Response::RPL_UNAWAY),
            "RPL_NOWAWAY" => Ok(Response::RPL_NOWAWAY),
            "306" => Ok(Response::RPL_NOWAWAY),
            "RPL_WHOISUSER" => Ok(Response::RPL_WHOISUSER),
            "311" => Ok(Response::RPL_WHOISUSER),
            "RPL_WHOISSERVER" => Ok(Response::RPL_WHOISSERVER),
            "312" => Ok(Response::RPL_WHOISSERVER),
            "RPL_WHOISOPERATOR" => Ok(Response::RPL_WHOISOPERATOR),
            "313" => Ok(Response::RPL_WHOISOPERATOR),
            "RPL_WHOISIDLE" => Ok(Response::RPL_WHOISIDLE),
            "317" => Ok(Response::RPL_WHOISIDLE),
            "RPL_ENDOFWHOIS" => Ok(Response::RPL_ENDOFWHOIS),
            "318" => Ok(Response::RPL_ENDOFWHOIS),
            "RPL_WHOISCHANNELS" => Ok(Response::RPL_WHOISCHANNELS),
            "319" => Ok(Response::RPL_WHOISCHANNELS),
            "RPL_WHOWASUSER" => Ok(Response::RPL_WHOWASUSER),
            "314" => Ok(Response::RPL_WHOWASUSER),
            "RPL_ENDOFWHOWAS" => Ok(Response::RPL_ENDOFWHOWAS),
            "369" => Ok(Response::RPL_ENDOFWHOWAS),
            "RPL_LISTSTART" => Ok(Response::RPL_LISTSTART),
            "321" => Ok(Response::RPL_LISTSTART),
            "RPL_LIST" => Ok(Response::RPL_LIST),
            "322" => Ok(Response::RPL_LIST),
            "RPL_LISTEND" => Ok(Response::RPL_LISTEND),
            "323" => Ok(Response::RPL_LISTEND),
            "RPL_CHANNELMODEIS" => Ok(Response::RPL_CHANNELMODEIS),
            "324" => Ok(Response::RPL_CHANNELMODEIS),
            "RPL_NOTOPIC" => Ok(Response::RPL_NOTOPIC),
            "331" => Ok(Response::RPL_NOTOPIC),
            "RPL_TOPIC" => Ok(Response::RPL_TOPIC),
            "332" => Ok(Response::RPL_TOPIC),
            "RPL_INVITING" => Ok(Response::RPL_INVITING),
            "341" => Ok(Response::RPL_INVITING),
            "RPL_SUMMONING" => Ok(Response::RPL_SUMMONING),
            "342" => Ok(Response::RPL_SUMMONING),
            "RPL_VERSION" => Ok(Response::RPL_VERSION),
            "351" => Ok(Response::RPL_VERSION),
            "RPL_WHOREPLY" => Ok(Response::RPL_WHOREPLY),
            "352" => Ok(Response::RPL_WHOREPLY),
            "RPL_ENDOFWHO" => Ok(Response::RPL_ENDOFWHO),
            "315" => Ok(Response::RPL_ENDOFWHO),
            "RPL_NAMREPLY" => Ok(Response::RPL_NAMREPLY),
            "353" => Ok(Response::RPL_NAMREPLY),
            "RPL_ENDOFNAMES" => Ok(Response::RPL_ENDOFNAMES),
            "366" => Ok(Response::RPL_ENDOFNAMES),
            "RPL_LINKS" => Ok(Response::RPL_LINKS),
            "364" => Ok(Response::RPL_LINKS),
            "RPL_ENDOFLINKS" => Ok(Response::RPL_ENDOFLINKS),
            "365" => Ok(Response::RPL_ENDOFLINKS),
            "RPL_BANLIST" => Ok(Response::RPL_BANLIST),
            "367" => Ok(Response::RPL_BANLIST),
            "RPL_ENDOFBANLIST" => Ok(Response::RPL_ENDOFBANLIST),
            "368" => Ok(Response::RPL_ENDOFBANLIST),
            "RPL_INFO" => Ok(Response::RPL_INFO),
            "371" => Ok(Response::RPL_INFO),
            "RPL_ENDOFINFO" => Ok(Response::RPL_ENDOFINFO),
            "374" => Ok(Response::RPL_ENDOFINFO),
            "RPL_MOTDSTART" => Ok(Response::RPL_MOTDSTART),
            "375" => Ok(Response::RPL_MOTDSTART),
            "RPL_MOTD" => Ok(Response::RPL_MOTD),
            "372" => Ok(Response::RPL_MOTD),
            "RPL_ENDOFMOTD" => Ok(Response::RPL_ENDOFMOTD),
            "376" => Ok(Response::RPL_ENDOFMOTD),
            "RPL_YOUREOPER" => Ok(Response::RPL_YOUREOPER),
            "381" => Ok(Response::RPL_YOUREOPER),
            "RPL_REHASHING" => Ok(Response::RPL_REHASHING),
            "382" => Ok(Response::RPL_REHASHING),
            "RPL_TIME" => Ok(Response::RPL_TIME),
            "391" => Ok(Response::RPL_TIME),
            "RPL_USERSSTART" => Ok(Response::RPL_USERSSTART),
            "392" => Ok(Response::RPL_USERSSTART),
            "RPL_USERS" => Ok(Response::RPL_USERS),
            "393" => Ok(Response::RPL_USERS),
            "RPL_ENDOFUSERS" => Ok(Response::RPL_ENDOFUSERS),
            "394" => Ok(Response::RPL_ENDOFUSERS),
            "RPL_NOUSERS" => Ok(Response::RPL_NOUSERS),
            "395" => Ok(Response::RPL_NOUSERS),
            "RPL_TRACELINK" => Ok(Response::RPL_TRACELINK),
            "200" => Ok(Response::RPL_TRACELINK),
            "RPL_TRACECONNECTING" => Ok(Response::RPL_TRACECONNECTING),
            "201" => Ok(Response::RPL_TRACECONNECTING),
            "RPL_TRACEHANDSHAKE" => Ok(Response::RPL_TRACEHANDSHAKE),
            "202" => Ok(Response::RPL_TRACEHANDSHAKE),
            "RPL_TRACEUNKNOWN" => Ok(Response::RPL_TRACEUNKNOWN),
            "203" => Ok(Response::RPL_TRACEUNKNOWN),
            "RPL_TRACEOPERATOR" => Ok(Response::RPL_TRACEOPERATOR),
            "204" => Ok(Response::RPL_TRACEOPERATOR),
            "RPL_TRACEUSER" => Ok(Response::RPL_TRACEUSER),
            "205" => Ok(Response::RPL_TRACEUSER),
            "RPL_TRACESERVER" => Ok(Response::RPL_TRACESERVER),
            "206" => Ok(Response::RPL_TRACESERVER),
            "RPL_TRACENEWTYPE" => Ok(Response::RPL_TRACENEWTYPE),
            "208" => Ok(Response::RPL_TRACENEWTYPE),
            "RPL_TRACELOG" => Ok(Response::RPL_TRACELOG),
            "261" => Ok(Response::RPL_TRACELOG),
            "RPL_STATSLINKINFO" => Ok(Response::RPL_STATSLINKINFO),
            "211" => Ok(Response::RPL_STATSLINKINFO),
            "RPL_STATSCOMMANDS" => Ok(Response::RPL_STATSCOMMANDS),
            "212" => Ok(Response::RPL_STATSCOMMANDS),
            "RPL_STATSCLINE" => Ok(Response::RPL_STATSCLINE),
            "213" => Ok(Response::RPL_STATSCLINE),
            "RPL_STATSNLINE" => Ok(Response::RPL_STATSNLINE),
            "214" => Ok(Response::RPL_STATSNLINE),
            "RPL_STATSILINE" => Ok(Response::RPL_STATSILINE),
            "215" => Ok(Response::RPL_STATSILINE),
            "RPL_STATSKLINE" => Ok(Response::RPL_STATSKLINE),
            "216" => Ok(Response::RPL_STATSKLINE),
            "RPL_STATSYLINE" => Ok(Response::RPL_STATSYLINE),
            "218" => Ok(Response::RPL_STATSYLINE),
            "RPL_ENDOFSTATS" => Ok(Response::RPL_ENDOFSTATS),
            "219" => Ok(Response::RPL_ENDOFSTATS),
            "RPL_STATSLLINE" => Ok(Response::RPL_STATSLLINE),
            "241" => Ok(Response::RPL_STATSLLINE),
            "RPL_STATSUPTIME" => Ok(Response::RPL_STATSUPTIME),
            "242" => Ok(Response::RPL_STATSUPTIME),
            "RPL_STATSOLINE" => Ok(Response::RPL_STATSOLINE),
            "243" => Ok(Response::RPL_STATSOLINE),
            "RPL_STATSHLINE" => Ok(Response::RPL_STATSHLINE),
            "244" => Ok(Response::RPL_STATSHLINE),
            "RPL_UMODEIS" => Ok(Response::RPL_UMODEIS),
            "221" => Ok(Response::RPL_UMODEIS),
            "RPL_LUSERCLIENT" => Ok(Response::RPL_LUSERCLIENT),
            "251" => Ok(Response::RPL_LUSERCLIENT),
            "RPL_LUSEROP" => Ok(Response::RPL_LUSEROP),
            "252" => Ok(Response::RPL_LUSEROP),
            "RPL_LUSERUNKNOWN" => Ok(Response::RPL_LUSERUNKNOWN),
            "253" => Ok(Response::RPL_LUSERUNKNOWN),
            "RPL_LUSERCHANNELS" => Ok(Response::RPL_LUSERCHANNELS),
            "254" => Ok(Response::RPL_LUSERCHANNELS),
            "RPL_LUSERME" => Ok(Response::RPL_LUSERME),
            "255" => Ok(Response::RPL_LUSERME),
            "RPL_ADMINME" => Ok(Response::RPL_ADMINME),
            "256" => Ok(Response::RPL_ADMINME),
            "RPL_ADMINLOC1" => Ok(Response::RPL_ADMINLOC1),
            "257" => Ok(Response::RPL_ADMINLOC1),
            "RPL_ADMINLOC2" => Ok(Response::RPL_ADMINLOC2),
            "258" => Ok(Response::RPL_ADMINLOC2),
            "RPL_ADMINEMAIL" => Ok(Response::RPL_ADMINEMAIL),
            "259" => Ok(Response::RPL_ADMINEMAIL),
            "RPL_TRACECLASS" => Ok(Response::RPL_TRACECLASS),
            "209" => Ok(Response::RPL_TRACECLASS),
            "RPL_STATSQLINE" => Ok(Response::RPL_STATSQLINE),
            "217" => Ok(Response::RPL_STATSQLINE),
            "RPL_SERVICEINFO" => Ok(Response::RPL_SERVICEINFO),
            "231" => Ok(Response::RPL_SERVICEINFO),
            "RPL_ENDOFSERVICES" => Ok(Response::RPL_ENDOFSERVICES),
            "232" => Ok(Response::RPL_ENDOFSERVICES),
            "RPL_SERVICE" => Ok(Response::RPL_SERVICE),
            "233" => Ok(Response::RPL_SERVICE),
            "RPL_SERVLIST" => Ok(Response::RPL_SERVLIST),
            "234" => Ok(Response::RPL_SERVLIST),
            "RPL_SERVLISTEND" => Ok(Response::RPL_SERVLISTEND),
            "235" => Ok(Response::RPL_SERVLISTEND),
            "RPL_WHOISCHANOP" => Ok(Response::RPL_WHOISCHANOP),
            "316" => Ok(Response::RPL_WHOISCHANOP),
            "RPL_KILLDONE" => Ok(Response::RPL_KILLDONE),
            "361" => Ok(Response::RPL_KILLDONE),
            "RPL_CLOSING" => Ok(Response::RPL_CLOSING),
            "362" => Ok(Response::RPL_CLOSING),
            "RPL_CLOSEEND" => Ok(Response::RPL_CLOSEEND),
            "363" => Ok(Response::RPL_CLOSEEND),
            "RPL_INFOSTART" => Ok(Response::RPL_INFOSTART),
            "373" => Ok(Response::RPL_INFOSTART),
            "RPL_MYPORTIS" => Ok(Response::RPL_MYPORTIS),
            "384" => Ok(Response::RPL_MYPORTIS),
            "ERR_YOUWILLBEBANNED" => Ok(Response::ERR_YOUWILLBEBANNED),
            "466" => Ok(Response::ERR_YOUWILLBEBANNED),
            "ERR_BADCHANMASK" => Ok(Response::ERR_BADCHANMASK),
            "476" => Ok(Response::ERR_BADCHANMASK),
            "ERR_NOSERVICEHOST" => Ok(Response::ERR_NOSERVICEHOST),
            "492" => Ok(Response::ERR_NOSERVICEHOST),
            "RPL_WELCOME" => Ok(Response::RPL_WELCOME { message: None }),
            "001" => Ok(Response::RPL_WELCOME { message: None }),
            "RPL_YOURHOST" => Ok(Response::RPL_YOURHOST),
            "002" => Ok(Response::RPL_YOURHOST),
            "RPL_CREATED" => Ok(Response::RPL_CREATED),
            "003" => Ok(Response::RPL_CREATED),
            "RPL_MYINFO" => Ok(Response::RPL_MYINFO),
            "004" => Ok(Response::RPL_MYINFO),
            "RPL_ISUPPORT" => Ok(Response::RPL_ISUPPORT),
            "005" => Ok(Response::RPL_ISUPPORT),
            "RPL_BOUNCE" => Ok(Response::RPL_BOUNCE),
            "010" => Ok(Response::RPL_BOUNCE),
            _ => Err(ParseError::new(
                ParseErrorKind::NotAResponse,
                "not a response",
            )),
        }
    }
}

impl str::FromStr for UserMode {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::super::{Message, Command, Request, Response};

    macro_rules! verify_parse{
        ($deserialized:expr, $raw:expr) => {
            assert_eq!($raw.parse::<Message>().unwrap(), $deserialized);
        }
    }

    #[test]
    fn test_parse() {
        verify_parse!(
            Message {
                prefix: Some("Laza".to_string()),
                command: Command::Req(Request::NICK { nickname: "lazau".to_string() }),
            },
            ":Laza NICK :lazau"
        );
    }
}
