echo '#![feature(plugin)]' > syscallmap.rs
echo '#![plugin(phf_macros)]' >> syscallmap.rs
echo '#[macro_use]' >> syscallmap.rs
echo 'extern crate phf;' >> syscallmap.rs
echo "pub static SYSCALL_LANGUAGE: phf::Map<&'static str, i64> = phf_map! {" >> syscallmap.rs

echo "#include<sys/syscall.h>" | gcc -dN -E - | awk '($1=="#define" && $2~/^SYS_/){print $2}' | cut -c 5- | cat -n | awk '{print "\t\""$2"\"=>", 20 + ($1 % 100)","}' >> syscallmap.rs

echo '};' >> syscallmap.rs
