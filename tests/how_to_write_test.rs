


#[cfg(test)]
mod tests
{
    #[test]
    fn test_add( )
    {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_match()
    {
        let number = 42;

        let answer = match number
        {
            42 => "All Answer of Universe",
            _ => "Nope."
        };

        assert_eq!( answer, "All Answer of Universe" );
    }

    
}
