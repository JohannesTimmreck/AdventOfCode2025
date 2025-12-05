#include <stdio.h>
#include <stdlib.h>

int turnDial(int currentNumber, int direction, int amount) {
    int newNumber = currentNumber + (direction * amount);

    if (newNumber > 99) {
        while (newNumber > 99) {
            newNumber -= 100;
        }
    } else if (newNumber < 0) {
        while (newNumber < 0) {
            newNumber = 100 + newNumber;
        }
    }
    return newNumber;
}

int main(int argv, char **argc) {
    int numbers[100]; 
    int Zeros = 0;
    int currentNumber = 50;
    int direction = 1;
    int amount = 0;
    
    FILE * fp;
    char * line = NULL;
    size_t len = 0;
    int lineNumber = 1;

    fp = fopen(argc[1], "r");
    if (fp == NULL)
        exit(EXIT_FAILURE);

    for (int i=0;i<100;i++)
        numbers[i] = 0;

    while (getline(&line, &len, fp) != -1) {
        printf("%d: %s", lineNumber++, line);
        if (line[0] == 'L') {
            direction = -1;
        } else if (line[0] == 'R') {
            direction = 1;
        }
        amount = atoi(&line[1]);
        while (amount > 100) {
            amount -= 100;
            numbers[0]++;
        }
        if (direction == 1) { 
            if (currentNumber != 0 && currentNumber + amount > 100) {
                numbers[0]++;
            }
        } else if (direction == -1) {
            if (currentNumber != 0 && currentNumber - amount < 0) {
                numbers[0]++;   
            }
        }
        int newNumber = currentNumber + (direction * amount);

        if (newNumber > 99) {
            newNumber -= 100;
        } else if (newNumber < 0) {
            newNumber = 100 + newNumber;
        }
        currentNumber = newNumber;
        printf("Current number: %d\n", currentNumber);
        numbers[currentNumber]++;
        printf("Amount of Zeros: %d\n", numbers[0]);
    }

    // for (int i=0;i<100;i++)
        // printf("Amount of %d: %d\n", i, numbers[i]);
    return (0);
}
