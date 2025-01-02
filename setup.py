from setuptools import setup, find_packages

setup(
    name="pyplus",
    version="0.1.0",
    author="bbyc",
    description="A collection of Python utility functions and extensions",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/yourusername/pyplus",  # 请替换为你的 GitHub 仓库地址
    packages=find_packages(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.6",
) 