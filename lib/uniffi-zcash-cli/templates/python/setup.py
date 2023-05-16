import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name="test-uniffi-zcash",
    version="{{version}}",
    author="zcash",
    description="Zcash librustzcash python FFI layer",
    long_description=long_description,
    long_description_content_type="text/markdown",
    packages=["zcash"],
    classifiers=[
        "Intended Audience :: Developers",
        "Topic :: Software Development :: Libraries",
        "Programming Language :: Python",
        "Programming Language :: C"
    ],                                     
    package_data={"zcash": ["libuniffi_zcash.so"]}
)