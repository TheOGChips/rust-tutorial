/* NOTE: Documents that which contains this comment
 * (usually used for crates and modules)
 */
//! # My Crate
//!
//! `publishing` is a collection of utilities to make performing
//! certain calculations more convenient.

//NOTE: Documents that which follows this comment (e.g. a function)
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = publishing::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/* NOTE: Other commonly used sections
 *      - Panics: The scenarios in which the function being
 *                documented could panic. Callers of the function
 *                who don’t want their programs to panic should
 *                make sure they don’t call the function in these
 *                situations.
 *      - Errors: If the function returns a Result, describing the
 *                kinds of errors that might occur and what
 *                conditions might cause those errors to be
 *                returned can be helpful to callers so they can
 *                write code to handle the different kinds of
 *                errors in different ways.
 *      - Safety: If the function is unsafe to call (we discuss
 *                unsafety in Chapter 19), there should be a
 *                section explaining why the function is unsafe and
 *                covering the invariants that the function expects
 *                callers to uphold.
 */
pub fn add_one (x: i32) -> i32 {
    return x + 1;
}
