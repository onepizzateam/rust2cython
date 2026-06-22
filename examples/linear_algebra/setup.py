from setuptools import setup
from Cython.Build import cythonize
from setuptools.extension import Extension
import numpy as np

extensions = [
    Extension(
        name="linear_algebra",
        sources=["linear_algebra.pyx"],
        libraries=["linear_algebra"],
        library_dirs=["."],
        include_dirs=[np.get_include()],
        extra_compile_args=["-O3"],
    )
]

setup(
    name="linear_algebra",
    ext_modules=cythonize(extensions, language_level="3"),
)
