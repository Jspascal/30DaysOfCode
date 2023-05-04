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

/**
 * Return binary number from base 10
 *
 * @param $number the number to convert
 *
 * @return array
 * **/
function convertToBinary(int $number): array
{
  $bi_number = [];
  while ($number > 1) {
    array_push($bi_number, $number % 2);
    $number = intdiv($number, 2);
    if ($number == 1) {
      array_push($bi_number, $number);
    }
  }
  return $bi_number;
}

/**
 * Return the longest streak of 1 in the binary number
 *
 * @param array $bin_number The binary number
 *
 * @return int
 * **/
function longestStreak(array $bin_number): int
{
  $tmp = $streak = 0;
  foreach ($bin_number as $value) {
    if ($value == 1) {
      $tmp++;
    }
    if ($value == 0 && $tmp >= $streak) {
      $streak = $tmp;
      $tmp = 0;
    }
    if ($value == 0 && $tmp < $streak) {
      $tmp = 0;
    }
  }
  if ($tmp > $streak) {
    $streak = $tmp;
  }
  return $streak;
}

$numb = (int)readline();
$binary_number = convertToBinary($numb);
$lnStreak = longestStreak($binary_number);
echo $lnStreak . PHP_EOL;
