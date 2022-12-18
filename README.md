# rustyt

## What is Rustyt?

Rustyt is a simple, fast, and easy to use parameter based converter for Youtube. It is written in Rust and it is using [youtube-dl](https://youtube-dl.org/).

### The Name

Name is a combination of Rust and Youtube. As you know, lot of project that written in Rust has a name that starts with _rusty_. So, I decided to use _rusty_ as the beggining  of name but also I want to make it related with Youtube. So, I decided to use _yt_ for the Youtube. And the result is _rustyt_.

## Why Rustyt?

There are a lot of Youtube downloader out there. But, I want to make a simple and easy to use downloader. So, I decided to make it with Rust. Rust is a fast language and it is easy to use (IMHO). So, I decided to make it in Rust.

## Docker ??

Docker containering the whole project. So, you can run it with docker whereever you are.

### GCP part of IT

I also want to make it related with GCP. So, I decided to integrate it with GCP. I'll use [GCP Cloud Runner](https://cloud.google.com/run) on that step cause this service is serverless so you don't need to build or maintain server. As a developer the price of hosting matters for me. Luckily, we have free tier on Cloud Runner. You can check the limits from price section on 
[website](https://cloud.google.com/run/pricing).

---

## How to use it?

### Container

#### Building a Docker Image

```bash
docker build -t rustyt .
```

#### Running a Docker Image

```bash
docker run --rm -i -t -p 8080:8080 rustyt
```

`note: first 8080 is a destination port, so you can change the port 8080 to whatever you want`

### Serverless

#### Google Cloud Platform

##### Cloud Runner

*TODO*