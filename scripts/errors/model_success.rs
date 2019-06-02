// source
// algorithms with rust L483

// rust works with option and result types that let developers
// model success or failure
// there is no exception system either, so any failed execution
// of a function should be indicated in the return type.

// only in rare cases when immediate termination is required 
// does the language provide panic!()

// handling those return values is often done with 
// match
// if let 

// this is due to Option<T> and Result<T,E> both being 
// enumerations that have generic types parameters
// they can assume any type in their variants
// matching on their variants provides access to their inner
// values and types to allow a branch of the code to be 
// executed and handle the case accordingly
// not only does this eliminate the need for constructs such 
// as try-catch with multiple execption arms, it makes
// failure part of the normal workflow that needs to be taken
// care of


