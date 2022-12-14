## Introduction

This is the collection of various applications for mechanics simulations and structural engineering models written in Rust. 

## Contents

### FluidCalc

Calculator for fluid flow functions (**&tau;(&lambda;)**, **&pi;(&lambda;)**, **&epsilon;(&lambda;)**, **q(&lambda;)**, **&phi;(&lambda;)** and **y(&lambda;)**) and reverse calculator for finding lambda parameter based on function value (**&lambda;(&tau;)**, **&lambda;(&pi;)**, **&lambda;(&epsilon;)**, **&lambda;(q)**, **&lambda;(&phi;)** and **&lambda;(y)**).

![FluidCalc direct functions screenshot](images/fluidcalc_direct.png)&nbsp;![FluidCalc reverse functions screenshot](images/fluidcalc_reverse.png)

### FluidView

Graphical solver for fluid flow **&lambda;<sub>1</sub>(q)** and **&lambda;<sub>2</sub>(q)** functions that uses [Newton's method](https://en.wikipedia.org/wiki/Newton%27s_method).

![FluidView screenshot](images/fluidview.png)

### BesselGraph

Graph plot of Bessel functions of the first kind of 0-th order **J<sub>0</sub>(x)** calculated with two methods: using integration and infinite series.

![BesselGraph screenshot](images/besselgraph.png)

## Links

* Original utils written in C++ &ndash; https://gitlab.com/Postrediori/OptimizationMethods
