### This program is used for calculating ethereum addresses in Classic, EIP55 and Contract forms
Private key when you insert
`xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx:xx`

and nonce 0..2<sup>64</sup>-1

## Installing
You need to install Python 3 from here https://www.python.org/ and Rust from here https://www.rust-lang.org/tools/install

Go to virtual environment in python 
```shell
python3 -m venv .env
source .env/bin/activate
```  
Install requirements
```shell
python -m pip install -r requirements.txt
```  
Build Rust library
```shell
maturin develop
``` 
For running 
```shell
python3 main.py
```