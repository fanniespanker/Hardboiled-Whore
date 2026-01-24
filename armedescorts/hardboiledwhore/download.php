<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE html>

<html>
  <head>
    <link rel="canonical" href="https://fanniespanker.com/armedescorts/hardboiledwhore/downloads/Hardboiled Whore - Fannie Spanker.epub"/>
    <!--<meta http-equiv="refresh" content="0; url="https://fanniespanker.com/armedescorts/hardboiledwhore/"/>-->
  </head>
  <body>
    <p>You should be downloading <a href="https://fanniespanker.com/armedescorts/hardboiledwhore/downloads/Hardboiled Whore - Fannie Spanker.epub">https://fanniespanker.com/armedescorts/hardboiledwhore/downloads/Hardboiled Whore - Fannie Spanker.epub</a>.</p>
  </body>
</html>


<?php

ini_set('display_errors', 0);
ini_set('log_errors', 'On');
ini_set('error_log', '/.logs/errors.log');

function emailError($errno, $errstr) {
  echo "<h1>An error has occurred.</h1><p>The site administrator has been notified.</p>";
  error_log("Error: [$errno] $errstr", 1, "fannie.spanker@gmail.com", "From: fanniespanker.com");
}
set_error_handler("emailError");

$filename = 'Hardboiled Whore - Fannie Spanker.epub';
$downloadfolder = 'downloads';
$downloadfilepath = "$downloadfolder/$filename";

$logs = '.logs';
$downloadlogpath = "$logs/downloads.log";


header('Content-Type: application/epub+zip');
header('Content-Transfer-Encoding: Binary');
header('Content-disposition: attachment; filename="' . basename($filename) . '"');

readfile($downloadfilepath);

$requesturi = $_SERVER['REQUEST_URI'];
$requesttime = date('Y-m-d H:i:s',$_SERVER['REQUEST_TIME']);
$remoteaddress = $_SERVER['REMOTE_ADDR'];
$useragent = $_SERVER['HTTP_USER_AGENT'];

$request = "http://ip-api.com/json/" . $remoteaddress . "?fields=22797881";
$response = file_get_contents($request);

$remoteaddresshash = hash("sha256", $remoteaddress);

$payload = join("\t", array( $requesttime, $remoteaddresshash, $useragent, $requesturi, $filename, $response) ) . PHP_EOL;

file_put_contents($downloadlogpath, $payload, FILE_APPEND);

?>