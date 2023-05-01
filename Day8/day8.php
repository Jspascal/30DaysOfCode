<?php

/**
 * Day 8 php implementation
 *
 * PHP version 8.1.3
 *
 * @category Training
 * @package  DaysOfCode
 * @author   Joseph Nomo <jspnomo@gmail.com>
 * @license  unlicensed https://unlicense.org 
 * @link     https://github.com/Jspascal/30DaysOfCode
 * */

$T = (int)readline();

$contacts = array();

for ($i = 0; $i < $T; $i++) {
    $contact = explode(" ", readline(), 2);
    $contacts[$contact[0]] = $contact[1];
}

for ($i = 0; $i < $T; $i++) {
    $contact_name = readline();
    if (array_key_exists($contact_name, $contacts)) {
        echo $contact_name . "=" . $contacts[$contact_name] . PHP_EOL;
    } else {
        echo "Not found" . PHP_EOL;
    }
}
