
use std::env;//imports the env module
use std::fs;//module used for file reading 
use std::process;
use std::error::Error;







fn main() {
    let args:Vec<String> =env::args().collect();

     
     //below is the use of the closure for err

    let config=Config::new(&args).unwrap_or_else(|err:&str|{
     println!("problem parsing arguments:{}",err);
     process::exit(1);

    }
    );

    println!("{}",config.query);
    println!("{}",config.filename);

      if let Err(e) =run(config)  {
          println!("application error:{}",e);
          process::exit(1);

      }




   
}

fn run(config:Config)->Result<(),Box<dyn Error>>
{
  
    let contents=fs::read_to_string(config.filename)?;


    println!("with text:\n{}",contents);

    Ok(())

}


struct Config{
    query:String,
    filename:String
}

impl Config{
    
    fn new(args:&Vec<String>)->Result<Config,&str>
{    
     if(args.len()<3)
     {
        return Err("not enough arguments");

     }

    let query=args[1].clone();
    let filename=args[2].clone();

    Ok(Config{query,filename})

    
}

}

