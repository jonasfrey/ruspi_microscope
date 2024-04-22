let a_s = f_o_command("lsusb")
    .s_stdout
    .split('\n')
    .filter(s=>s.trim()!='')
let n = parseInt(prompt(
    `select an input:${a_s.map(
        (s,n_idx)=>`${parseInt(n_idx)+1}:${s}`
    )}`
));
let s = a_s[n-1];
let s_vidpid = s.split(" ")[5];
let n_vid = parseInt(s_vidpid.split(":").shift(),16);
let n_pid = parseInt(s_vidpid.split(":").pop(),16);