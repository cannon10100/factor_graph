#!/bin/bash

mkdir graphs
sudo docker build -t factor_graph ../
sudo docker run -v `pwd`/graphs:/graphs factor_graph
