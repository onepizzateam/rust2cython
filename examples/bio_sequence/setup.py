from setuptools import setup
from Cython.Build import cythonize
from setuptools.extension import Extension
import numpy as np

extensions = [
    Extension(
        name="bio_sequence",
        sources=["bio_sequence.pyx"],
        libraries=["bio_sequence"],
        library_dirs=["."],
        include_dirs=[np.get_include()],
        extra_compile_args=["-O3"],
    )
]

setup(
    name="bio_sequence",
    ext_modules=cythonize(extensions, language_level="3"),
)
