use crate::parser::Parser;

pub fn pair<'a, R1, R2>(parser1: Parser<'a, R1>, parser2: Parser<'a, R2>) -> Parser<'a, (R1,R2)>
where
    R1: 'a,
    R2: 'a,
{
     
    Parser::new(
        move | input | parser1.parse(input).and_then(| (next_input, result1) | 
            parser2.parse(next_input).map(| (final_input,result2) | (final_input, (result1, result2))) 
        )
        
    ) 
    //parser1.bind(move| result1 | parser2.map(move | result2 | (result1,result2)))
}