#!/bin/bash
docker build -t rados_list_small:0.1 .
docker build -f Dockerfile-dev -t rados_list:0.1 .
docker rm rados_list
docker run -it -v /media/nfs_share/git/rados-list/status:/app/status -v /media/nfs_share/git/rados-list/bucketlists:/app/bucketlists rados_list:0.1 /app/bucketlists/bucket_list_1.txt
