 
# Although libmobi has now been added as a submodule, make sure to clean the libmobi between builds
# Since leftover data can interfere with subsequent builds

# Clone the latest libmobi
#  rm -rfd ./libmobi 
# git clone https://github.com/bfabiszewski/libmobi.git


# Begin Patching

cp -r "./libmobi - mod/Mod/libmobi/" ./
sed -i 's/\$(top_builddir)/../g' ./libmobi/tools/Makefile.am # The sed command is needed for ancient builds of autotools. 

# Build Application
cd ./libmobi
./autogen.sh && ./configure --with-libxml2=no --with-zlib=no && make # The autogen line will build libmobi
cd ..

# Copy built modified libmobitool to linux folder
cp ./libmobi/tools/libmobitoolmod.a ./libmobi-rs/libs/linux/

# Run unit tests

cd ./libmobi-rs
cargo test -- --nocapture

cd ..
