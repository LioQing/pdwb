#include <stdio.h>
#include <stdlib.h>
#include <time.h>



int main(int argc, char** argv) {
    int lower = 1, upper = 100;
    int n;
    int input = 0;

    srand(time(NULL));
    n = rand() % 100 + 1;

    while(input != n){
        printf("Make a guess (%d - %d): ", lower, upper);
        scanf("%d", &input);

        if (input > n) {
            printf("%d is too large\n", input);
            upper = input - 1;
        } else if (input < n) {
            printf("%d is too small\n", input);
            lower = input + 1;
        } else {
            printf("You are correct\n");
        }
    }

    return 0;
}