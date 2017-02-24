with Ada.Text_IO; 
use Ada.Text_IO;
with Palindrome;

procedure Main is
    res : Boolean := False;
    test : String(1..1024);
    len : Integer;
begin
    Get_Line(test, len);
    
    res := Palindrome(test(1..len));
    if res then
        Put_Line("true");
    else
        Put_Line("false");
    end if;
end Main;
