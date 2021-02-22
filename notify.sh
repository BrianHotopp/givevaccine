#!/bin/sh
curl -X POST --data "apikey=<I removed my api key>" --data "priority=high" --data "event=UALBANY Vaccine Available" --data "application=Vaccine Notifier" https://api.prowlapp.com/publicapi/add
