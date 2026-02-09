//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

pub fn doubtful(s: &mut String) {
    s.push('?')
}

/*
    * Q1 : this `s` string is in this format :
    - Stack:                        Heap:
    ┌─────────────┐              ┌───────────────────────┐
    │ s (String)  │              │ 'hello' (5 bytes)     │
    ├─────────────┤              ├───────────────────────┤
    │ ptr ────────┼─────────────→ │ (rest of allocated)   │
    │ len = 5     │              │ (capacity bytes)      │
    │ capacity=8  │              └───────────────────────┘
    └─────────────┘
    ?so we are working with the data on the heap, and we are modifying it by pushing a '?' character to the end of the string, which will change the string to "hello?" and update the length to 6, but the capacity will remain the same at 8.
    ?low level wise the push method will check if there is enough capacity to add the new character, if there is it will simply add the character to the end of the string and update the length, if there isn't it will need to allocate more memory on the heap to accommodate the new character, copy the existing data to the new location, and then add the new character.
    ?also searching for the end of the string to add the new character is done by using the length metadata, so it will directly access the memory location at ptr + len to add the new character, which is why it's important to keep the length updated correctly.
*/