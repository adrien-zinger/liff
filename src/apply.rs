//fn print(msg: &Vec<u8>) -> String {
//  let dest: Vec::<String> = msg.iter().map(|u| (*u as char).to_string()).collect();
//  dest.join("")
//}

type Diff<T> = Vec<(u8, u32, u32, Vec<T>, Vec<T>)>;

#[allow(clippy::all)]
pub fn apply(mut source: Vec<u8>, diff: &Diff<u8>) -> Vec<u8> {
    //println!("source: {}", source.len());
    for operation in diff.iter() {
        //println!("source: {}", print(&source));
        let position = operation.1 as usize;
        if operation.0 == 0 {
            let mut dest = operation.4.clone();
            dest.reverse();
            //println!("Replace from {} to {}, '{}'", position, position + dest.len(), print(&dest));
            source.splice(position..position + dest.len(), dest);
        }
        if operation.0 == 1 {
            let range = position..position;
            let mut dest = operation.3.clone();
            dest.reverse();
            //println!("Add at {}, '{}'", position, print(&dest));
            source.splice(range, dest);
        }
        if operation.0 == 2 {
            //println!("Delete from {} to {}", position, position + operation.2 as usize);
            source.splice(position..position + operation.2 as usize, Vec::new());
        }
    }
    source
}
