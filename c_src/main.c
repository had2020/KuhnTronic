#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "time.h"
#include <unistd.h>
#include "nnet.c"
#include "kuhnStates.c"

int main(int argc, char *argv[]) {

    struct Model model = {};
    init(model);

    struct dataset dataset = {};

    /*
    struct dataset.training_inputs = {
        {0.0f, 0.0f},
        {1.0f, 0.0f},
        {0.0f, 1.0f},
        {1.0f, 1.0f}
    };*/

    // Training for XOR
    double training_inputs[numTrainingSets][numInputs] = {
        {0.0f, 0.0f},
        {1.0f, 0.0f},
        {0.0f, 1.0f},
        {1.0f, 1.0f}
    };
    **dataset.training_inputs = **training_inputs;

    for (int i = 0; i < numTrainingSets; i++ ) {
        printf("idx: [%d] = %f", i, dataset.training_inputs[1][i]);
    }


    if (argc == 2) {
        printf("Raw debugging mode enabled. Input first State as a number: \n");

        while(1) {
            int input_s;
            scanf("%d", &input_s);
            printf("%d\n", node_tree[input_s]);
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
