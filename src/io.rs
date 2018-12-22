use super::*;

#[no_mangle]
pub unsafe extern fn openDevice(device: *const c_char) -> c_int {
  let device = CStr::from_ptr(device).to_str().unwrap();

  let fd;

  if !device.starts_with("/") && device.contains(":") {
    let parts: Vec<_> = device.splitn(2, ':').collect();

    let hostname = String::from(parts[0]);
    let port: c_int = match parts[1].parse() {
      Ok(port) => port,
      _ => return -1,
    };

    fd = openCliSocket(hostname.as_ptr() as _, port, 1);
  } else {
    fd = opentty(device.as_ptr() as _);
  }

  if fd < 0 {
    return -1
  }

  fd
}

#[no_mangle]
pub unsafe extern fn closeDevice(fd: c_int) {
  close(fd);
}

#[no_mangle]
pub unsafe extern fn openCliSocket(host: *const c_char, port: c_int, noTCPdelay: c_int) -> c_int {
  let host = CStr::from_ptr(host);

  let mut hints: addrinfo = mem::zeroed();
  hints.ai_family   = PF_UNSPEC as _;
  hints.ai_socktype = SOCK_STREAM as _;
  hints.ai_flags    = AI_ALL | AI_V4MAPPED;

  let mut res: *mut addrinfo = ptr::null_mut();

  let port_string = CString::new(port.to_string()).unwrap();

  let n = getaddrinfo(host.as_ptr(), port_string.into_raw(), &hints as *const addrinfo, &mut res as *mut *mut addrinfo);

  if n < 0 {
    let error_message = CStr::from_ptr(gai_strerror(n)).to_str().unwrap();
    log_it!(LOG_ERR, "Error in getaddrinfo: {}:{}", host.to_str().unwrap(), error_message);
    exit(1);
  }

  let ressave = res.clone();

  let mut sockfd = -1;

  while let Some(curr_res) = res.as_ref() {
    sockfd = socket(curr_res.ai_family, curr_res.ai_socktype, curr_res.ai_protocol);

    if sockfd >= 0 {
      if connect(sockfd, curr_res.ai_addr as *mut _, curr_res.ai_addrlen) == 0 {
        break
      }

      close(sockfd);
      sockfd = -1;
    }

    res = curr_res.ai_next;
  }

  freeaddrinfo(ressave);

  if sockfd < 0 {
    log_it!(LOG_ERR, "TTY Net: No connection to {}:{}", host.to_str().unwrap(), port);
    return -1
  }

  log_it!(LOG_INFO, "ClI Net: connected {}:{} (FD:{})", host.to_str().unwrap(), port, sockfd);

  let flag: *const c_int = &1;

  if noTCPdelay != 0 && setsockopt(sockfd, IPPROTO_TCP as _, TCP_NODELAY, flag as *const c_void, mem::size_of_val(&*flag) as u32) != 0 {
    log_it!(LOG_ERR, "Error in setsockopt TCP_NODELAY ({})", std::io::Error::last_os_error());
  }

  sockfd
}