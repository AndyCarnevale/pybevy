## pybevy - using Bevy in Python
An experiment attempting to expose the [Bevy game engine](https://github.com/bevyengine/bevy) to Python, using [PyO3](https://github.com/PyO3/pyo3).

At the moment, this is just a learning project and not suitable for use in an actual project.


## Motivation
### Why use Bevy in Python?
* Access to an ECS framework
* Access to Python's REPL during prototyping and initial play-testing

### Personal
* Learn how py03 works


## Instructions
[Maturin](https://github.com/PyO3/maturin) is used to compile changes to your Rust code.
First set up a Python virtual environment:
```
python -m venv venv
```
Then activate the environment. On Windows:
```
.\venv\Scripts\activate
``` 
Then install the Rust package into the virtual environment:
```
maturin develop --release
```
Now the Rust library is available to be imported in Python.

For example, if the Rust library were named `pybevy`, it could be imported in Python as:
```
import pybevy

pybevy.do_something()
```
