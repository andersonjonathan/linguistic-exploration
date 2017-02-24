function Palindrome (message: String) return Boolean is
-- Palindrome checker

rev, orig : String(message'First..message'Last);
counter, rev_count : Integer := 0;

begin
    -- Remove any spaces
    for c in message'First..message'Last loop
        if message(c) /= ' ' then
            counter := counter + 1;
            orig(counter..counter) := message(c..c);
        end if;
    end loop;

    -- Reverse the string
    for d in reverse 1..counter loop
        rev_count := rev_count + 1;
        rev(rev_count..rev_count) := orig(d..d);
    end loop;

    -- Compare and return
    return orig(1..counter) = rev(1..counter);

end Palindrome;
