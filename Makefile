PI_IP =192.168.1.208

all:
	clear

	# build binary
	cargo build --target armv7-unknown-linux-gnueabihf
	
	# upload binary
	sshpass -p 'raspberry' scp -r ./target/armv7-unknown-linux-gnueabihf/debug/matrix_rhal pi@$(PI_IP):/home/pi
	
	# execute binary
	sshpass -p 'raspberry' ssh pi@$(PI_IP) './matrix_rhal'
