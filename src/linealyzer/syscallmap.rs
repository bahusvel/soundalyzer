use phf;

pub static SYSCALL_LANGUAGE: phf::Map<&'static str, u8> =
    phf_map! {
	"_sysctl"=> 21,
	"accept"=> 22,
	"accept4"=> 23,
	"access"=> 24,
	"acct"=> 25,
	"add_key"=> 26,
	"adjtimex"=> 27,
	"afs_syscall"=> 28,
	"alarm"=> 29,
	"arch_prctl"=> 30,
	"bind"=> 31,
	"bpf"=> 32,
	"brk"=> 33,
	"capget"=> 34,
	"capset"=> 35,
	"chdir"=> 36,
	"chmod"=> 37,
	"chown"=> 38,
	"chroot"=> 39,
	"clock_adjtime"=> 40,
	"clock_getres"=> 41,
	"clock_gettime"=> 42,
	"clock_nanosleep"=> 43,
	"clock_settime"=> 44,
	"clone"=> 45,
	"close"=> 46,
	"connect"=> 47,
	"copy_file_range"=> 48,
	"creat"=> 49,
	"create_module"=> 50,
	"delete_module"=> 51,
	"dup"=> 52,
	"dup2"=> 53,
	"dup3"=> 54,
	"epoll_create"=> 55,
	"epoll_create1"=> 56,
	"epoll_ctl"=> 57,
	"epoll_ctl_old"=> 58,
	"epoll_pwait"=> 59,
	"epoll_wait"=> 60,
	"epoll_wait_old"=> 61,
	"eventfd"=> 62,
	"eventfd2"=> 63,
	"execve"=> 64,
	"execveat"=> 65,
	"exit"=> 66,
	"exit_group"=> 67,
	"faccessat"=> 68,
	"fadvise64"=> 69,
	"fallocate"=> 70,
	"fanotify_init"=> 71,
	"fanotify_mark"=> 72,
	"fchdir"=> 73,
	"fchmod"=> 74,
	"fchmodat"=> 75,
	"fchown"=> 76,
	"fchownat"=> 77,
	"fcntl"=> 78,
	"fdatasync"=> 79,
	"fgetxattr"=> 80,
	"finit_module"=> 81,
	"flistxattr"=> 82,
	"flock"=> 83,
	"fork"=> 84,
	"fremovexattr"=> 85,
	"fsetxattr"=> 86,
	"fstat"=> 87,
	"fstatfs"=> 88,
	"fsync"=> 89,
	"ftruncate"=> 90,
	"futex"=> 91,
	"futimesat"=> 92,
	"get_kernel_syms"=> 93,
	"get_mempolicy"=> 94,
	"get_robust_list"=> 95,
	"get_thread_area"=> 96,
	"getcpu"=> 97,
	"getcwd"=> 98,
	"getdents"=> 99,
	"getdents64"=> 100,
	"getegid"=> 101,
	"geteuid"=> 102,
	"getgid"=> 103,
	"getgroups"=> 104,
	"getitimer"=> 105,
	"getpeername"=> 106,
	"getpgid"=> 107,
	"getpgrp"=> 108,
	"getpid"=> 109,
	"getpmsg"=> 110,
	"getppid"=> 111,
	"getpriority"=> 112,
	"getrandom"=> 113,
	"getresgid"=> 114,
	"getresuid"=> 115,
	"getrlimit"=> 116,
	"getrusage"=> 117,
	"getsid"=> 118,
	"getsockname"=> 119,
	"getsockopt"=> 20,
	"gettid"=> 21,
	"gettimeofday"=> 22,
	"getuid"=> 23,
	"getxattr"=> 24,
	"init_module"=> 25,
	"inotify_add_watch"=> 26,
	"inotify_init"=> 27,
	"inotify_init1"=> 28,
	"inotify_rm_watch"=> 29,
	"io_cancel"=> 30,
	"io_destroy"=> 31,
	"io_getevents"=> 32,
	"io_setup"=> 33,
	"io_submit"=> 34,
	"ioctl"=> 35,
	"ioperm"=> 36,
	"iopl"=> 37,
	"ioprio_get"=> 38,
	"ioprio_set"=> 39,
	"kcmp"=> 40,
	"kexec_file_load"=> 41,
	"kexec_load"=> 42,
	"keyctl"=> 43,
	"kill"=> 44,
	"lchown"=> 45,
	"lgetxattr"=> 46,
	"link"=> 47,
	"linkat"=> 48,
	"listen"=> 49,
	"listxattr"=> 50,
	"llistxattr"=> 51,
	"lookup_dcookie"=> 52,
	"lremovexattr"=> 53,
	"lseek"=> 54,
	"lsetxattr"=> 55,
	"lstat"=> 56,
	"madvise"=> 57,
	"mbind"=> 58,
	"membarrier"=> 59,
	"memfd_create"=> 60,
	"migrate_pages"=> 61,
	"mincore"=> 62,
	"mkdir"=> 63,
	"mkdirat"=> 64,
	"mknod"=> 65,
	"mknodat"=> 66,
	"mlock"=> 67,
	"mlock2"=> 68,
	"mlockall"=> 69,
	"mmap"=> 70,
	"modify_ldt"=> 71,
	"mount"=> 72,
	"move_pages"=> 73,
	"mprotect"=> 74,
	"mq_getsetattr"=> 75,
	"mq_notify"=> 76,
	"mq_open"=> 77,
	"mq_timedreceive"=> 78,
	"mq_timedsend"=> 79,
	"mq_unlink"=> 80,
	"mremap"=> 81,
	"msgctl"=> 82,
	"msgget"=> 83,
	"msgrcv"=> 84,
	"msgsnd"=> 85,
	"msync"=> 86,
	"munlock"=> 87,
	"munlockall"=> 88,
	"munmap"=> 89,
	"name_to_handle_at"=> 90,
	"nanosleep"=> 91,
	"newfstatat"=> 92,
	"nfsservctl"=> 93,
	"open"=> 94,
	"open_by_handle_at"=> 95,
	"openat"=> 96,
	"pause"=> 97,
	"perf_event_open"=> 98,
	"personality"=> 99,
	"pipe"=> 100,
	"pipe2"=> 101,
	"pivot_root"=> 102,
	"pkey_alloc"=> 103,
	"pkey_free"=> 104,
	"pkey_mprotect"=> 105,
	"poll"=> 106,
	"ppoll"=> 107,
	"prctl"=> 108,
	"pread64"=> 109,
	"preadv"=> 110,
	"preadv2"=> 111,
	"prlimit64"=> 112,
	"process_vm_readv"=> 113,
	"process_vm_writev"=> 114,
	"pselect6"=> 115,
	"ptrace"=> 116,
	"putpmsg"=> 117,
	"pwrite64"=> 118,
	"pwritev"=> 119,
	"pwritev2"=> 20,
	"query_module"=> 21,
	"quotactl"=> 22,
	"read"=> 23,
	"readahead"=> 24,
	"readlink"=> 25,
	"readlinkat"=> 26,
	"readv"=> 27,
	"reboot"=> 28,
	"recvfrom"=> 29,
	"recvmmsg"=> 30,
	"recvmsg"=> 31,
	"remap_file_pages"=> 32,
	"removexattr"=> 33,
	"rename"=> 34,
	"renameat"=> 35,
	"renameat2"=> 36,
	"request_key"=> 37,
	"restart_syscall"=> 38,
	"rmdir"=> 39,
	"rt_sigaction"=> 40,
	"rt_sigpending"=> 41,
	"rt_sigprocmask"=> 42,
	"rt_sigqueueinfo"=> 43,
	"rt_sigreturn"=> 44,
	"rt_sigsuspend"=> 45,
	"rt_sigtimedwait"=> 46,
	"rt_tgsigqueueinfo"=> 47,
	"sched_get_priority_max"=> 48,
	"sched_get_priority_min"=> 49,
	"sched_getaffinity"=> 50,
	"sched_getattr"=> 51,
	"sched_getparam"=> 52,
	"sched_getscheduler"=> 53,
	"sched_rr_get_interval"=> 54,
	"sched_setaffinity"=> 55,
	"sched_setattr"=> 56,
	"sched_setparam"=> 57,
	"sched_setscheduler"=> 58,
	"sched_yield"=> 59,
	"seccomp"=> 60,
	"security"=> 61,
	"select"=> 62,
	"semctl"=> 63,
	"semget"=> 64,
	"semop"=> 65,
	"semtimedop"=> 66,
	"sendfile"=> 67,
	"sendmmsg"=> 68,
	"sendmsg"=> 69,
	"sendto"=> 70,
	"set_mempolicy"=> 71,
	"set_robust_list"=> 72,
	"set_thread_area"=> 73,
	"set_tid_address"=> 74,
	"setdomainname"=> 75,
	"setfsgid"=> 76,
	"setfsuid"=> 77,
	"setgid"=> 78,
	"setgroups"=> 79,
	"sethostname"=> 80,
	"setitimer"=> 81,
	"setns"=> 82,
	"setpgid"=> 83,
	"setpriority"=> 84,
	"setregid"=> 85,
	"setresgid"=> 86,
	"setresuid"=> 87,
	"setreuid"=> 88,
	"setrlimit"=> 89,
	"setsid"=> 90,
	"setsockopt"=> 91,
	"settimeofday"=> 92,
	"setuid"=> 93,
	"setxattr"=> 94,
	"shmat"=> 95,
	"shmctl"=> 96,
	"shmdt"=> 97,
	"shmget"=> 98,
	"shutdown"=> 99,
	"sigaltstack"=> 100,
	"signalfd"=> 101,
	"signalfd4"=> 102,
	"socket"=> 103,
	"socketpair"=> 104,
	"splice"=> 105,
	"stat"=> 106,
	"statfs"=> 107,
	"statx"=> 108,
	"swapoff"=> 109,
	"swapon"=> 110,
	"symlink"=> 111,
	"symlinkat"=> 112,
	"sync"=> 113,
	"sync_file_range"=> 114,
	"syncfs"=> 115,
	"sysfs"=> 116,
	"sysinfo"=> 117,
	"syslog"=> 118,
	"tee"=> 119,
	"tgkill"=> 20,
	"time"=> 21,
	"timer_create"=> 22,
	"timer_delete"=> 23,
	"timer_getoverrun"=> 24,
	"timer_gettime"=> 25,
	"timer_settime"=> 26,
	"timerfd_create"=> 27,
	"timerfd_gettime"=> 28,
	"timerfd_settime"=> 29,
	"times"=> 30,
	"tkill"=> 31,
	"truncate"=> 32,
	"tuxcall"=> 33,
	"umask"=> 34,
	"umount2"=> 35,
	"uname"=> 36,
	"unlink"=> 37,
	"unlinkat"=> 38,
	"unshare"=> 39,
	"uselib"=> 40,
	"userfaultfd"=> 41,
	"ustat"=> 42,
	"utime"=> 43,
	"utimensat"=> 44,
	"utimes"=> 45,
	"vfork"=> 46,
	"vhangup"=> 47,
	"vmsplice"=> 48,
	"vserver"=> 49,
	"wait4"=> 50,
	"waitid"=> 51,
	"write"=> 52,
	"writev"=> 53,
};

pub fn get_note(text: &str) -> Option<u8> {
    SYSCALL_LANGUAGE.get(text).cloned()
}