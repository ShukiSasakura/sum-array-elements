# set variable
RUNTIME="wasmer"
TARGET="./target/wasm32-wasmer-wasi/release/sum-array-elements.wasm"
LENGTH=1000000
REPEAT=100000
LOG_FILE="./logs/sum.log"

# invoke program
for i in 1 #2 4 10 20 100
do
    for j in 1 10 100 1000
    do
        THREAD_NUM=$i
        CHUNK_SIZE=$j
        $RUNTIME $TARGET -- -l $LENGTH -c $CHUNK_SIZE -t $THREAD_NUM -r $REPEAT >> $LOG_FILE
    done
done
