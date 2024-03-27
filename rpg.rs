fn readl() -> String{
	use std::io::stdin;
	let mut buffer = String::new();
	stdin().read_line(&mut buffer).unwrap();
	return buffer;
}
const empty:u8=46;
const block:u8=35;
const spike:u8=94;
const chest:u8=66;
const enamy:u8=38;
const boss:u8=77;
const me:u8=64;
const dl:u8=76;
const dr:u8=82;
const du:u8=85;
const dd:u8=68;

pub fn main() {
	let mut nm=readl();
	let mut nm=nm
		.split_ascii_whitespace()
		.flat_map(str::parse::<usize>);
	let n=nm.next().unwrap();
	let m=nm.next().unwrap();
	let mut b=vec![[block;105];105];
	let mut name=vec![vec![String::new();105];105]; 
	let mut atk=vec![[0;105];105];
	let mut def=vec![[0;105];105];
	let mut hp=vec![[0;105];105];
	let mut exp=vec![[0;105];105];
	for i in 1..=n{
		let lin=readl();
		for sj in lin.trim().bytes().enumerate(){
            b[i][sj.0+1]=sj.1;
        }
	}
	let instruction=readl();

	for i in 1..=n{
		for j in 1..=m{
			if b[i][j]==enamy||b[i][j]==boss{
				let mut lin=readl();
				let mut slin=lin.split_ascii_whitespace();
				let x=str::parse::<usize>(slin.next().unwrap()).unwrap();
				let y=str::parse::<usize>(slin.next().unwrap()).unwrap();
				name[x][y]=slin.next().unwrap().to_string();
				atk[x][y]=str::parse::<i32>(slin.next().unwrap()).unwrap();
				def[x][y]=str::parse::<i32>(slin.next().unwrap()).unwrap();
				hp[x][y]=str::parse::<i32>(slin.next().unwrap()).unwrap();
				exp[x][y]=str::parse::<i32>(slin.next().unwrap()).unwrap();
			}
		}
	}
	for i in 1..=n{
		for j in 1..=m{
			if b[i][j]==chest{
				let mut lin=readl();
				let mut slin=lin.split_ascii_whitespace();
				let x=str::parse::<usize>(slin.next().unwrap()).unwrap();
				let y=str::parse::<usize>(slin.next().unwrap()).unwrap();
				name[x][y]=slin.next().unwrap().to_string();
				if name[x][y]=="O"{
					name[x][y]=slin.next().unwrap().to_string();
				}else{
					exp[x][y]=str::parse::<i32>(slin.next().unwrap()).unwrap();
				}
			}
		}
	}
	let mut x:usize=0;
	let mut y:usize=0;
	let mut myhp:i32=20;
	let mut lvl:i32=1;
	let mut myexp:i32=0;
	let mut atk_:i32=0;
	let mut def_:i32=0;
	let mut special:Vec<String>=Vec::new();
	let mut state:String="Press any key to continue.".to_string();
	let mut turn_cnt=0;
	for i in 1..=n{
		for j in 1..=m{
			if b[i][j]==me{
				b[i][j]=empty;
				x=i;
				y=j;
			}
		}
	}
	let sx=x;
	let sy=y;
	for si in instruction.trim().bytes().enumerate(){
		turn_cnt=si.0+1;
		let mut nx=x;
		let mut ny=y;
		match si.1{
			dl=>ny-=1,
			dr=>ny+=1,
			du=>nx-=1,
			dd=>nx+=1,
			_=>return
		}
		if b[nx][ny]!=block{
			x=nx;
			y=ny;
		}
		match b[x][y]{
			spike=>{
				if special.contains(&"DX".to_string()){
					myhp-=1;
				}else{
					myhp-=5;
				}
				if myhp<=0{
					state="YOU HAVE BEEN KILLED BY SPIKE TRAP..".to_string();
				}
			},
			chest=>{
				match name[x][y].as_str(){
					"W"=>atk_=exp[x][y],
					"A"=>def_=exp[x][y],
					_=>{
						if special.len()<4&&!(special.contains(&name[x][y])){
							let mut o=String::from(&name[x][y]);
							special.push(o);
						}
					}
				}
				b[x][y]=empty;
			},
			enamy|boss=>{
				let natk=lvl*2+atk_;
				let ndef=lvl*2+def_;
				let yatk=atk[x][y];
				let ydef=def[x][y];
				let mut yhp=hp[x][y];
				if b[x][y]==boss&&special.contains(&"HU".to_string()){
					myhp=lvl*5+15;
				}
				if special.contains(&"CO".to_string()){
					if special.contains(&"DX".to_string()){
						yhp-=std::cmp::max(1,natk*3-ydef);
					}else{
						yhp-=std::cmp::max(1,natk*2-ydef);
					}
				}else{
					yhp-=std::cmp::max(1,natk-ydef);
				}
				if b[x][y]==boss&&special.contains(&"HU".to_string()){
					yhp-=std::cmp::max(1,natk-ydef);
				}
				while yhp>0{
					myhp-=std::cmp::max(1,yatk-ndef);
					if myhp<=0{
						break;
					}
					yhp-=std::cmp::max(1,natk-ydef);
				}
				if myhp<=0{
					state="YOU HAVE BEEN KILLED BY ".to_string()+&name[x][y]+"..";
				}else{
					myexp+=exp[x][y];
					if special.contains(&"EX".to_string()){
						myexp+=exp[x][y]/5;
					}
					if special.contains(&"HR".to_string()){
						myhp=std::cmp::min(lvl*5+15,myhp+3);
					}
					if myexp>=lvl*5{
						lvl+=1;
						myexp=0;
						myhp=lvl*5+15;
					}
					if b[x][y]==boss{
						state="YOU WIN!".to_string();
						break;
					}
					b[x][y]=empty;
				}
			},
			_=>{}
		}
		if myhp<=0{
			if special.contains(&"RE".to_string()){
				let idx=special.iter().position(|x|*x=="RE".to_string()).unwrap();
				special.remove(idx);
				x=sx;
				y=sy;
				myhp=lvl*5+15;
				state="Press any key to continue.".to_string();
			}else{
				myhp=0;
				break;
			}
		}
		if state=="YOU WIN!".to_string(){
			break;
		}
	}
	if myhp>0{
		b[x][y]=me;
	}
	for i in 1..=n{
		for j in 1..=m{
			print!("{}",std::char::from_u32(b[i][j]as u32).unwrap());
		}
		println!("");
	}
	println!("Passed Turns : {}",turn_cnt);
	println!("LV : {}",lvl);
	println!("HP : {}/{}",myhp,lvl*5+15);
	println!("ATT : {}+{}",lvl*2,atk_);
	println!("DEF : {}+{}",lvl*2,def_);
	println!("EXP : {}/{}",myexp,lvl*5);
	println!("{}",state);
}
