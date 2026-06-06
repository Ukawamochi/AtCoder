if arr[i + 1][j] == tar && arr[i + 1][j + 1] == tar && arr[i][j + 1] == tar && arr[i - 1][j + 1] == tar && arr[i - 1][j] == tar && arr[i - 1][j - 1] == tar && arr[i][j - 1] == tar && arr[i + 1][j - 1] == tar {
    flag = 1;
}