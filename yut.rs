fn readl() -> String{
	use std::io::stdin;
	let mut buffer = String::new();
	stdin().read_line(&mut buffer).unwrap();
	return buffer;
}

pub fn main() {
	//let mut n=readl();
	let a:[[usize;5];30]=[[1,2,3,4,5],[2,3,4,5,6],[3,4,5,6,7],[4,5,6,7,8],[5,6,7,8,9],[21,22,23,24,25],[7,8,9,10,11],[8,9,10,11,12],[9,10,11,12,13],[10,11,12,13,14],[26,27,23,28,29],[12,13,14,15,16],[13,14,15,16,17],[14,15,16,17,18],[15,16,17,18,19],[16,17,18,19,20],[17,18,19,20,0],[18,19,20,0,0],[19,20,0,0,0],[20,0,0,0,0],[0,0,0,0,0],[22,23,24,25,15],[23,24,25,15,16],[28,29,20,0,0],[25,15,16,17,18],[15,16,17,18,19],[27,23,28,29,20],[23,28,29,20,0],[29,20,0,0,0],[20,0,0,0,0]];
	let w:[usize;30]=[0,822,624,426,228,30,24,18,12,6,0,198,396,594,792,990,996,1002,1008,1014,1020,190,350,510,670,830,170,340,680,850];
	let ww:[usize;32]=[0,0,1,33,34,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
	let mut o:[u8;1056]=[46,46,45,45,45,45,46,46,45,45,45,45,46,46,45,45,45,45,46,46,45,45,45,45,46,46,45,45,45,45,46,46,10,46,46,32,32,32,32,46,46,32,32,32,32,46,46,32,32,32,32,46,46,32,32,32,32,46,46,32,32,32,32,46,46,10,124,32,92,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,47,32,124,10,124,32,32,92,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,47,32,32,124,10,124,32,32,32,92,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,47,32,32,32,124,10,124,32,32,32,32,46,46,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,46,46,32,32,32,32,124,10,46,46,32,32,32,46,46,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,46,46,32,32,32,46,46,10,46,46,32,32,32,32,32,92,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,47,32,32,32,32,32,46,46,10,124,32,32,32,32,32,32,32,92,32,32,32,32,32,32,32,32,32,32,32,32,32,32,47,32,32,32,32,32,32,32,124,10,124,32,32,32,32,32,32,32,32,92,32,32,32,32,32,32,32,32,32,32,32,32,47,32,32,32,32,32,32,32,32,124,10,124,32,32,32,32,32,32,32,32,32,46,46,32,32,32,32,32,32,32,32,46,46,32,32,32,32,32,32,32,32,32,124,10,124,32,32,32,32,32,32,32,32,32,46,46,32,32,32,32,32,32,32,32,46,46,32,32,32,32,32,32,32,32,32,124,10,46,46,32,32,32,32,32,32,32,32,32,32,92,32,32,32,32,32,32,47,32,32,32,32,32,32,32,32,32,32,46,46,10,46,46,32,32,32,32,32,32,32,32,32,32,32,92,32,32,32,32,47,32,32,32,32,32,32,32,32,32,32,32,46,46,10,124,32,32,32,32,32,32,32,32,32,32,32,32,32,92,32,32,47,32,32,32,32,32,32,32,32,32,32,32,32,32,124,10,124,32,32,32,32,32,32,32,32,32,32,32,32,32,32,46,46,32,32,32,32,32,32,32,32,32,32,32,32,32,32,124,10,124,32,32,32,32,32,32,32,32,32,32,32,32,32,32,46,46,32,32,32,32,32,32,32,32,32,32,32,32,32,32,124,10,124,32,32,32,32,32,32,32,32,32,32,32,32,32,47,32,32,92,32,32,32,32,32,32,32,32,32,32,32,32,32,124,10,46,46,32,32,32,32,32,32,32,32,32,32,32,47,32,32,32,32,92,32,32,32,32,32,32,32,32,32,32,32,46,46,10,46,46,32,32,32,32,32,32,32,32,32,32,47,32,32,32,32,32,32,92,32,32,32,32,32,32,32,32,32,32,46,46,10,124,32,32,32,32,32,32,32,32,32,46,46,32,32,32,32,32,32,32,32,46,46,32,32,32,32,32,32,32,32,32,124,10,124,32,32,32,32,32,32,32,32,32,46,46,32,32,32,32,32,32,32,32,46,46,32,32,32,32,32,32,32,32,32,124,10,124,32,32,32,32,32,32,32,32,47,32,32,32,32,32,32,32,32,32,32,32,32,92,32,32,32,32,32,32,32,32,124,10,124,32,32,32,32,32,32,32,47,32,32,32,32,32,32,32,32,32,32,32,32,32,32,92,32,32,32,32,32,32,32,124,10,46,46,32,32,32,32,32,47,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,92,32,32,32,32,32,46,46,10,46,46,32,32,32,46,46,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,46,46,32,32,32,46,46,10,124,32,32,32,32,46,46,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,46,46,32,32,32,32,124,10,124,32,32,32,47,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,92,32,32,32,124,10,124,32,32,47,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,92,32,32,124,10,124,32,47,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,92,32,124,10,46,46,32,32,32,32,46,46,32,32,32,32,46,46,32,32,32,32,46,46,32,32,32,32,46,46,32,32,32,32,46,46,10,46,46,45,45,45,45,46,46,45,45,45,45,46,46,45,45,45,45,46,46,45,45,45,45,46,46,45,45,45,45,46,46,10];
	let mut pos:[usize;256]=[0;256];
	let n:usize=readl().trim().parse::<usize>().unwrap();
	for _i in 0..n{
		let lin=readl();
		let mut slin=lin.split_ascii_whitespace();
		let x=slin.next().unwrap().to_string();
		let x:usize=x.trim().bytes().next().unwrap() as usize;
		let y=slin.next().unwrap().to_string();
		let mut c:usize=0;
		for yi in y.trim().bytes().enumerate(){
			if yi.1==70{
				c=c+1;
			}
		}
		c=if c==0{4}else{c-1};
		let p:usize=pos[x];
		let q:usize=a[p][c];
		let u:usize=if x<96{65}else{97};
		let v:usize=if x<96{97}else{65};
		for b in u..u+4{
			if b==x||(p!=0&&pos[b]==p){
				pos[b]=q;
			}
		}
		for b in v..v+4{
			if pos[b]==q{
				pos[b]=0;
			}
		}
	}
	for b in 0..256{
		if pos[b]!=0{
			o[w[pos[b]]+ww[b%32]]=b as u8;
		}
	}
	for i in 0..1056{
		print!("{}",std::char::from_u32(o[i]as u32).unwrap());
	}
}
