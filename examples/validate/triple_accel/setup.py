from setuptools import setup
from Cython.Build import cythonize
from setuptools.extension import Extension
import numpy as np

import sys
if sys.platform == 'darwin':
    rpath_arg = "-Wl,-rpath,@loader_path"
else:
    rpath_arg = "-Wl,-rpath,$ORIGIN"


extensions = [
    Extension(
        name="triple_accel",
        sources=["triple_accel.pyx"],
        libraries=["triple_accel"],
        library_dirs=["."],
        include_dirs=[np.get_include()],
        extra_compile_args=["-O3"],
        extra_link_args=[rpath_arg],
    )
]

setup(
    name="triple_accel",
    ext_modules=cythonize(extensions, language_level="3"),
)
