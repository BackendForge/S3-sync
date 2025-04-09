# Rust for S3

!Requires Rust installed.

Create directory **status** when using with wrapper. This is intended and require manual intervention when adding to wrapper.

Docker image after build will include **rados_list** binary file & **.env** file in `/app`.

Docker image has built in **ENTRYPOINT**, so pass after image **ONLY** args.

Ex.: `docker run -it -v /mnt/status:/app/status -v /mnt/bucketlists:/app/bucketlists -v /mnt/datetime:/app/datetime rados_list_small:0.1 /app/bucketlists/bucket_list_1.txt /app/datetime/datetime.txt`

## Local testing

Copy **.env.sample** to **.env** and put correct variables.

To compile & run debug: `cargo run bucketlists/bucket_list_1.txt datetime/datetime.txt`

To compile faster program: `cargo build --release`

Usage: `[program] [bucket list file path] [full path to datetime file]`. There is example file `bucket_list_1.txt` and `datetime.txt`.

## Docker

Copy **.env.sample** to **.env** and put correct variables.

To build image: `docker build -t rados_list_small:0.1 .`

To export image: `docker save rados_list_small:0.1 > /tmp/rados_list_small.tar`

To load image: `docker load < /tmp/rados_list_small.tar`

To build & test container locally: `./start.sh`
