from setuptools import (find_packages,
                        setup)

setup(name="gitmergedeps", packages=find_packages(),
      install_requires=["requirements-parser==0.2.0"])
