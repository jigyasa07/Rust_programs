#include <iostream>
#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <netinet/in.h> 
#include <arpa/inet.h>
#include <sys/epoll.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

#define DEFAULT_PORT    3000
#define MAX_CONN        40000
#define MAX_EVENTS      80000

using namespace std;

static void set_sockaddr(struct sockaddr_in *addr)
{
	bzero((char *)addr, sizeof(struct sockaddr_in));
	addr->sin_family = AF_INET;
	addr->sin_addr.s_addr = INADDR_ANY;
	addr->sin_port = htons(DEFAULT_PORT);
}

static int setnonblocking(int sockfd)
{
	if (fcntl(sockfd, F_SETFL, fcntl(sockfd, F_GETFD, 0) | O_NONBLOCK) ==
	    -1) {
		return -1;
	}
	return 0;
}

int main()
{
    int i,n;
    int epfd,nfds;
    int listen_sock,conn_sock;
    socklen_t socklen;
    char buf[512];
    struct sockaddr_in srv_addr;
    struct sockaddr_in cli_addr;
    struct epoll_event ev, events[MAX_EVENTS];
    ev.events = EPOLLIN;
    ev.data.fd = listen_sock;

    listen_sock = socket(AF_INET,SOCK_STREAM,0);

    set_sockaddr(&srv_addr);
    bind(listen_sock,(struct sockaddr *)&srv_addr, sizeof(srv_addr));

    setnonblocking(listen_sock);
    listen(listen_sock,MAX_CONN);

    epfd = epoll_create(2);
    if(epfd == -1)
    {
        cout<<"epoll create error"<<endl;
    }
    epoll_ctl(epfd,EPOLL_CTL_ADD,listen_sock,&ev);

    socklen = sizeof(cli_addr);

    int c=1;

    for(;;)
    {
        nfds = epoll_wait(epfd,events,MAX_EVENTS,-1);
        //cout<<nfds<<endl;
        for(i=0;i<nfds;++i)
        {
            
            if(events[i].data.fd == listen_sock)
            {
                conn_sock = accept(listen_sock,(struct sockaddr *)&cli_addr,&socklen);

                inet_ntop(AF_INET,(char*)&(cli_addr.sin_addr),buf,sizeof(cli_addr));
                cout << c << " Got client:"<< buf<< ntohs(cli_addr.sin_port) << endl;
                c=c+1;

                ev.events = EPOLLIN | EPOLLET | EPOLLRDHUP |EPOLLHUP;
                ev.data.fd = conn_sock;

                setnonblocking(conn_sock);
                epoll_ctl(epfd,EPOLL_CTL_ADD, conn_sock, &ev);
            }
             else if (events[i].events & EPOLLIN)
             {
                for(;;)
                {
                    bzero(buf,sizeof(buf));
                    n = read(events[i].data.fd,buf,sizeof(buf));

                    if(n<=0)
                    {
                        break;
                    }
                    else
                    {
                        cout<<"data recieved: "<<(n)<<endl;
                        //write(events[i].data.fd,buf,strlen(buf));
                    }
                }
              // epoll_ctl_add(epfd,events[i].data.fd, EPOLLIN | EPOLLET |EPOLLONESHOT );
             }
            else
                {
                    cout<<"error"<<endl;
                }
        }
    }
}