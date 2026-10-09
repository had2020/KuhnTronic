#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "time.h"
#include <unistd.h>
#include <vector>
#include "nnet.cpp"
#include "kuhnStates.cpp"

int main(int argc, char *argv[]) {

    struct Model model = {};
    init(model);

    struct dataset dataset;

    // TODO API 
    dataset.training_inputs.reserve(32);
    dataset.training_outputs.reserve(32);
    dataset.training_inputs.push_back({1.0, 0.0, 1.0, 0.0});
    dataset.training_outputs.push_back({2.0});
    train(10000, 0.1, model, dataset);


    if (argc == 2) {
        printf("Raw debugging mode enabled. Input first State as a number: \n");

        while(1) {
            int input_s;
            if (scanf("%d", &input_s)) {
                printf("%d\n", node_tree[input_s]);
            } else {
                printf("Input failed to read!");
                break;
            }

        }

    } else {
        FILE* fptr;

        fptr = fopen("../temp.swp", "r");

        if (fptr == NULL) {
            printf("Error openning file!");
        } else {
            fclose(fptr);
        }


        printf("The C interface is running, press control+c to kill it.\n");
        while (1) {
            usleep(1000000);
            break; //TODO remove break, and communicate
        }
    }

    return 0;
}
