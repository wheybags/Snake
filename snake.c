/* Controls: Up, Down, Left, and Right, P to pause, Q to quit */

#include <unistd.h> 
#include <ncurses.h>
#include <stdlib.h>
#include <time.h>


#define FX 36
#define FY 20

#define true	1
#define false	0

#define UP	0
#define DOWN	1
#define LEFT	2
#define RIGHT	3

#define	PLAYING	0
#define	PAUSED	1
#define	DEAD	2

void clearmid();
void placepoint(int xsnake[], int ysnake[], int playerx, int playery); 
int rnumber(int lower, int upper);

int fieldx = FX;
int fieldy = FY;
char field[FY][FX];


int main(int argc, const char *argv[]){
	
	/* ncurses initialisation stuff */
	initscr();
	noecho();
	keypad(stdscr, TRUE);
	timeout(0);

	int i, j;
	/* Clear Playing Field */
	for(i = 0; i != fieldy; i++){
		for(j = 0; j != fieldx; j++){
			field[i][j] = '#';
		}
	}
	clearmid();

	/* Add in Newlines so field can be printed as one string */
	for(i = 0; i != fieldy - 1; i++){
		field[i][fieldx - 1] = '\n';
	}
	field[fieldy - 1][fieldx - 1] = '\0';	// Null terminator

	int area = fieldx * fieldy;
	int length = 2;	// length does not include head
	char headchar;	// The current head character (^,v,<,>)
	int dir = RIGHT;
	int playerx = fieldx / 2;
	int playery = fieldy / 2;
	int place = true;
	int status = PLAYING;

	/* Initialises the player */	
	int xsnake[area + 1];	// Stores x/y values for snake tail
	int ysnake[area + 1];	// Size is area + 1, so that the snake can never exceed the array.
	ysnake[0] = ysnake[1] = ysnake[2] = playery;
	xsnake[0] = playerx - 2;
	xsnake[1] = playerx - 1;
	xsnake[2] = playerx;
	field[playery][playerx -1] = field[playery][playerx] = '*';
	int snakeindex = 2;
	
	int again = true;	
	int c;
	while(again){

		c = 0;
		c = getch();
		switch(c){
			case KEY_UP:{
				if(dir != DOWN && status == PLAYING) dir = UP;
				break;
			}
			case KEY_DOWN:{
				if(dir != UP && status == PLAYING) dir = DOWN;
				break;
			}
			case KEY_LEFT:{
				if(dir != RIGHT && status == PLAYING) dir = LEFT;
				break;
			}
			case KEY_RIGHT:{
				if(dir != LEFT && status == PLAYING) dir = RIGHT;
				break;
			}
			case 'q':{
				again = false;
				break;
			}
			case 'p':{
				if(status == PLAYING){	
					status = PAUSED;
					move(fieldy / 2, fieldx / 2 - 3);
					printw("PAUSED");
				}
				else if(status == PAUSED){
					status = PLAYING;
				}
				break;
			}
			default:{
				break;
			}
		}
		switch(status){
			case PLAYING:{
				field[playery][playerx] = '*';
				switch(dir){
					case UP:{
						playery--;
						headchar = '^';
						break;
					}
					case DOWN:{
						playery++;
						headchar = 'v';
						break;
					}
					case LEFT:{
						playerx--;
						headchar = '<';
						break;
					}
					case RIGHT:{
						playerx++;
						headchar = '>';
						break;
					}
				}
				switch(field[playery][playerx]){
					case '@':{
						length++;
						place = true;
					}
					case ' ':{
						/* Removes the end of the snake */
						i = snakeindex - length;
						if(i < 0) i = area + 1 + i;
						field[ysnake[i]][xsnake[i]] = ' ';
						
						/* Adds the current position to the tail arrays */				
						snakeindex = (snakeindex + 1) % (area + 1);
						xsnake[snakeindex] = playerx;
						ysnake[snakeindex] = playery;
						field[playery][playerx] = headchar;	// Draws head
						if(place){
							placepoint(xsnake, ysnake, playerx, playery);
							place = false;
						}
						break;
					}
					default:{
						status = DEAD;
						field[ysnake[snakeindex]][xsnake[snakeindex]] = headchar;
						break;
					}
				}
				
				/* These three lines actually draw the frame */
				move(3,0);
				printw("%s", field);
				move(1,(fieldx/2) - 7); printw("Score: %d", length - 2);
				//move(3,0); printw("%d,%d  \"%c\"", x, y, field[y][x]);
				refresh();
				break;
			}
			default:{
				break;
			}
		}
		
		usleep(80000); // Sets framerate, basically.
	}
	endwin();
	return 0;
}

void placepoint(int xsnake[], int ysnake[], int playerx, int playery){
	int again = true;
	int x, y;
	for(x = rnumber(1, fieldx - 3), y = rnumber(1, fieldy - 3); field[y][x] != ' '; x = rnumber(1, fieldx - 3), y = rnumber(1, fieldy - 3));
	field[y][x] = '@';
}

int rnumber(int lower, int upper){
	time_t seconds;
	time(&seconds);
	srand((unsigned int) seconds);
	return (rand() % (upper - lower + 1)) + lower;
}

void clearmid(){
	int i, j;
	for(i = 1; i != fieldy -1; i++){
		for(j = 1; j != fieldx - 2; j++){
			field[i][j] = ' ';
		}
	}
}
