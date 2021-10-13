from setuptools import setup
from version import VERSION

TEST_DEPS = [
    'pytest<3.7', 'pytest-asyncio==0.10.0', 'base58'
]

setup(
    name='vdr-tools',
    version=VERSION,
    packages=['indy'],
    url='https://github.com/evernym/vdr-tools',
    license='Apache-2.0',
    author='Evernym',
    author_email='dev@evernym.com',
    description='A library that facilitates building standards compliant and interoperable solutions for self-sovereign identity by abstracting the operations for interacting with a verifiable data registry as defined by Hyperledger Aries.',
    install_requires=['base58'],
    tests_require=TEST_DEPS,
    extras_require={
        'test': TEST_DEPS
    }
)
