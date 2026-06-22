from setuptools import setup
from Cython.Build import cythonize
from setuptools.extension import Extension
import numpy as np

extensions = [
    Extension(
        name="simple",
        sources=["simple.pyx"],
        libraries=["simple"],
        library_dirs=["."],
        include_dirs=[np.get_include()],
        extra_compile_args=["-O3"],
    )
]

setup(
    name="simple",
    ext_modules=cythonize(extensions, language_level="3"),
)
