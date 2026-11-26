// SPDX-License-Identifier: MIT
// Generate static slider tables from GigaChess's own deterministic multipliers.
mod magic { include!("scripts/own_magics.rs"); }
use std::{env,fs::File,io::{BufWriter,Write},path::PathBuf};
fn ray(s:usize,occ:u64,diag:bool,mask:bool)->u64 {
    let dirs = if diag {[(1,1),(1,-1),(-1,1),(-1,-1)]} else {[(1,0),(-1,0),(0,1),(0,-1)]};
    let mut out=0;
    for (df,dr) in dirs {
        let (mut f,mut r)=(s as i32%8+df,s as i32/8+dr);
        while (0..8).contains(&f)&&(0..8).contains(&r) {
            if mask && (!(0..8).contains(&(f+df)) || !(0..8).contains(&(r+dr))) {break;}
            let b=1u64<<(r*8+f);out|=b;
            if occ&b!=0 {break;}
            f+=df;r+=dr;
        }
    }
    out
}
fn table(w:&mut impl Write,name:&str,diag:bool,magics:&[u64;64],pext:bool){
    let mut desc=Vec::new();let mut data=Vec::new();
    for (sq,&magic) in magics.iter().enumerate(){
        let mask=ray(sq,0,diag,true);let bits=mask.count_ones();let shift=64-bits;
        let base=data.len();data.resize(base+(1usize<<bits),0);
        desc.push((mask,magic,shift,base));
        let mut occ=0u64;let mut index=0usize;
        loop{
            let idx=if pext{index}else{(occ.wrapping_mul(magic)>>shift)as usize};
            let att=ray(sq,occ,diag,false);
            assert!(data[base+idx]==0||data[base+idx]==att,"magic collision");
            data[base+idx]=att;
            index+=1;occ=occ.wrapping_sub(mask)&mask;if occ==0{break;}
        }
    }
    writeln!(w,"pub static {name}_DESC: [(u64,u64,u32,usize);64] = {desc:?};").unwrap();
    writeln!(w,"pub static {name}_DATA: [u64;{}] = [",data.len()).unwrap();
    for chunk in data.chunks(8){for x in chunk{write!(w,"{x},").unwrap();}writeln!(w).unwrap();}
    writeln!(w,"];").unwrap();
}
fn main(){
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=scripts/own_magics.rs");
    if env::var_os("CARGO_FEATURE_EXP_STATIC_SLIDERS").is_none(){return;}
    let path=PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("static_sliders.rs");
    let mut w=BufWriter::new(File::create(path).unwrap());
    // Static PEXT is used only if BMI2 is a compile-time target feature.
    let pext=env::var_os("CARGO_FEATURE_PEXT").is_some()&&env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default().split(',').any(|x|x=="bmi2");
    table(&mut w,"ROOK",false,&magic::ROOK_MAGICS,pext);
    table(&mut w,"BISHOP",true,&magic::BISHOP_MAGICS,pext);
}
