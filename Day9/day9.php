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

function factorial(int $number)
{
  $limit = $number;
  for ($i = 1; $i < $limit; $i++) {
    $number *= $i;
  }
  return $number;
}

$number = (int)readline();
$number = factorial($number);
echo $number . PHP_EOL;
