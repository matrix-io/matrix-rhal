all:
	clear

	# build binary
	cargo build --target armv7-unknown-linux-gnueabihf
	
	# upload binary
	sshpass -p 'raspberry' scp -r ./target/armv7-unknown-linux-gnueabihf/debug/matrix_rhal pi@192.168.2.199:/home/pi
	
	# execute binary
	sshpass -p 'raspberry' ssh pi@192.168.2.199 './matrix_rhal'