// Small fixed sized like integers can be stored directly into the code but
// for larger values like strings get stored in a separate "contant data" region
// We'll be using a similiar approach as the Java VM, where each chunk will carry
// a list of values (a constant pool)
