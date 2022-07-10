use lindera::tokenizer::Tokenizer;
use lindera::LinderaResult;

mod vector;
use vector::tf_idf;

fn main() -> LinderaResult<()> {
    // create tokenizer
    let tokenizer = Tokenizer::new()?;

    // tokenize the text
    let tokens = tokenizer.tokenize("オススメの曲ありますか？")?;

    // output the tokens
    for token in tokens {
        println!("{}", token.text);
    }


    let mut docs = Vec::new();
    let doc1 = vec!["犬", "可愛い", "犬", "大きい"];
    let doc2 = vec!["猫", "小さい", "猫", "可愛い", "可愛い"];
    let doc3 = vec!["虫", "小さい", "可愛くない"];
    
    docs.push(doc1);
    docs.push(doc2);
    docs.push(doc3);

    Ok(())


    
}