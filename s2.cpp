#include <iostream>
#include <stdio.h>
#include <stdlib.h>
#include <strings.h>
#include <string>
#include <sstream>
#include <vector>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include "pthread.h"

struct thread_args {
    int newsockfd1;
	sockaddr_in *cli_addr1;
};

void *handleConnection(void *arguments) {
	// int newsockfd = arguments->newsockfd1;
	// sockaddr cli_addr = arguments ->cli_addr1;
	struct thread_args *args = (struct thread_args *)arguments;
	char buffer[512];
	bzero(buffer, 512);
	//std::cout<<"got client"<<std::endl;
	while (true) {
		int n = read(args->newsockfd1, buffer, 512);
		//std::cout<<"No of bytes: "<<n-1<<std::endl;

		std::stringstream stream;
		stream << buffer << std::flush;
		while (stream.good()) {
			std::string request;
			getline(stream, request); 
			if (request.length() > 0) {
				std::cout << inet_ntoa(args->cli_addr1->sin_addr) << ":" << ntohs(args->cli_addr1->sin_port)
						<< ": " << request << std::endl;
				//std::cout <<"No of bytes: "<<request.length()<<std::endl;
			}
		}
	}
}

int main(int argc, const char *argv[]) {
	int sockfd; 
	int portno; 
	pthread_t thread_id;
	sockaddr_in serv_addr;

	sockfd = socket(AF_INET, SOCK_STREAM, 0); 
	if (sockfd < 0) {
		std::cerr << "ERROR opening socket" << std::endl;
	}

	int reusePort = 1; 
	setsockopt(sockfd, SOL_SOCKET, SO_REUSEPORT, &reusePort, sizeof(reusePort));

	bzero((char *) &serv_addr, sizeof(serv_addr)); 
	portno = 3000;

	serv_addr.sin_family = AF_INET; 
	serv_addr.sin_port = htons(portno);
	serv_addr.sin_addr.s_addr = INADDR_ANY; 
	if (bind(sockfd, (sockaddr *) &serv_addr, sizeof(serv_addr)) < 0)
		std::cerr << "Error on binding" << std::endl;

	unsigned int backlogSize = 10000; 
	listen(sockfd, backlogSize);
	std::cout << "C++ server opened on port " << portno << std::endl;;

	while (true) {
		int newsockfd;
		unsigned int clilen; 
		sockaddr_in cli_addr; 

		struct thread_args args;
		args.newsockfd1 = newsockfd;
		args.cli_addr1 = &cli_addr;

		clilen = sizeof(sockaddr_in);
		newsockfd = accept(sockfd, (sockaddr *) &cli_addr, &clilen); 
		if (newsockfd < 0)
			std::cerr << "Error on accept" << std::endl;

		std::cout << inet_ntoa(cli_addr.sin_addr) << ":" << ntohs(cli_addr.sin_port)
				<< " connected" << std::endl;

		pthread_create (&thread_id,NULL,&handleConnection, (void *)&args) ; 
		//handleConnection.join();
	}
	pthread_join(thread_id,NULL);
	int q;
	std::cin>>q;
   

}