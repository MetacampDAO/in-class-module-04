// 1. create a struct Unicorn with fields:
    // name of type String
    // magic_powers of type u32

// 2. create a struct Griffin with fields:
    // name of type String
    // magic_powers of type u32

// 5. create an enum Creature with Variants:
    // Unicorn with data of (Unicorn)
    // Griffin with data of (Griffin)

// 8. implement onto Creature
    // 9. define magic_power function that takes in a referenced self and outputs unsigned 32bits integer
        // matches on itself
            // if its of variant Unicorn, return referenced unicorn's magic power
            // if its of variant Griffin, return referenced griffin's magic power
    
    // 12. define name function that takes in a referenced self and outputs a string slice
        // matches on itself
            // if its of variant Unicorn, return referenced unicorn's name
            // if its of variant Griffin, return referenced griffin's name

    // 13. define clone function which takes in a referenced self and returns Self/Creature (interchangeable)
        // matches on itself
            // if its of variant Unicorn, return a Unicorn variant
                // name should be assigned to the output of the previously defined name function above
                // magic_power should be assigned to the output of the previously defined magic_power function above
            // if its of variant Griffin, return a Griffin variant
                // name should be assigned to the output of the previously defined name function above
                // magic_power should be assigned to the output of the previously defined magic_power function above

// 10. define compare_magic function, takes in 2 referenced creatures and return 1 referenced creature (lifetime!!)
    // if first creature has larger output derived from magic_power function
        // return first creature
    // else
        // return second creature

// 14. define creature_box function, takes in 1 referenced creature and returns box with an owned creature (lifetime!!)
    // return a box containing an owned creature by calling the clone function within  the box

    fn main() {
        // 3. create a variable unicorn of Unicorn instance
    
        // 4. create a variable griffin of Griffin instance
    
        // 6. create variable unicorn_creature, a Creature enum with Unicorn variant
        // 7. create variable griffin_creature, a Creature enum with Griffin variant
    
        // 11. create stronger_creature, the output of compare_magic function, passing in unicorn and griffin creature
        // 15. create boxed_creature, the output of creature_box function, passing in stronger creature
    
        // 16. print "The stronger creature is <creature name> with a magic power of <creature power>."
    }
    