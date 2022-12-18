# rust-apps

Running a local socat server

 socat -v tcp-l:1234,fork exec:'/bin/cat'