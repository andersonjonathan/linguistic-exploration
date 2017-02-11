#!/usr/bin/env bash

function is_palindrome {
   input=$1
   res=$2
   original=${input//[[:space:]]/}  # Remove spaces

   for((i=${#original}-1; i>=0; i--));  # reverse string
       do reverse="${reverse}${original:$i:1}";
   done

   if [ "${reverse}" == "${original}" ]; then  # Compare string
       eval ${res}="true"
   else
       eval ${res}="false"
   fi
}

read text
is_palindrome "${text}" result
echo ${result}
exit